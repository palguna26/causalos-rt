use tracing::{info, instrument};
use sidecar::ipc::windows::run_named_pipe_server;
use sidecar::engine::governance::GovernanceEngine;
use sidecar::engine::diagnostics::{DiagnosticEngine, OutcomeClass};
use sidecar::engine::promotion::PromotionManager;
use sidecar::engine::ranking::Ranker;
use sidecar::engine::trace::TraceEngine;
use sidecar::engine::simulation::{HybridSimulator, SimulationVerdict};
use sidecar::storage::ledger::CausalLedger;
use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::RwLock;
use std::sync::Arc;
use std::env;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod kernel_proto {
    tonic::include_proto!("causalos.kernel");
}

use kernel_proto::kernel_service_server::{KernelService, KernelServiceServer};
pub use kernel_proto::*;

pub struct KernelHost {
    governance: GovernanceEngine,
    diagnostics: DiagnosticEngine,
    promotion: PromotionManager,
    trace_engine: TraceEngine,
    simulator: HybridSimulator,
    ranker: Arc<RwLock<Ranker>>,
    ledger: Arc<RwLock<CausalLedger>>,
}

#[tonic::async_trait]
impl KernelService for KernelHost {
    async fn evaluate_plan(
        &self,
        request: Request<PlanRequest>,
    ) -> Result<Response<PlanContract>, Status> {
        let req = request.into_inner();
        let (risk_score, invariants) = self.governance.evaluate_plan(&req.plan_text);

        let contract = PlanContract {
            contract_hash: format!("plan_{:x}", fxhash::hash64(&req.plan_text)),
            risk_score,
            required_invariants: invariants.into_iter().map(|name| {
                Invariant { name, condition: "CHECK_REQUIRED".to_string() }
            }).collect(),
            watchpoints: vec![],
        };

        Ok(Response::new(contract))
    }

    async fn record_outcome(
        &self,
        request: Request<OutcomeRequest>,
    ) -> Result<Response<OutcomeResponse>, Status> {
        let req = request.into_inner();
        info!("Recording outcome: success={}", req.success);

        let outcome_class = self.diagnostics.analyze_outcome(&req.details, req.success);
        
        match outcome_class {
            OutcomeClass::DeterministicSuccess => {
                info!("Promoting verified success to CNS.");
                let mut ledger = self.ledger.write().await;
                let _ = self.promotion.promote_to_ledger(&mut ledger, &req.success_criteria, &req.details);
                
                let mut ranker = self.ranker.write().await;
                ranker.apply_reinforcement(&req.plan_hash, 0.1);
            },
            OutcomeClass::CausalFailure => {
                info!("Applying negative reinforcement for failure.");
                let mut ranker = self.ranker.write().await;
                ranker.apply_reinforcement(&req.plan_hash, -0.2);
            },
            _ => {}
        }

        Ok(Response::new(OutcomeResponse { recorded: true }))
    }

    async fn prepare_tool_call(
        &self,
        request: Request<ToolCallRequest>,
    ) -> Result<Response<ToolCallVerdict>, Status> {
        let req = request.into_inner();
        
        // 1. Static Governance
        let verdict = self.governance.evaluate_tool_call(&req.tool_name, &req.arguments_json);
        
        // 2. Causal Hybrid Simulation
        let args: serde_json::Value = serde_json::from_str(&req.arguments_json).unwrap_or_default();
        let ledger = self.ledger.read().await;
        let sim_result = self.simulator.simulate(&req.tool_name, &args, &ledger);

        use kernel_proto::tool_call_verdict::Action;
        let (action, reason) = match sim_result {
            SimulationVerdict::Success => {
                use sidecar::engine::governance::Action as GAction;
                match verdict.action {
                    GAction::Allow => (Action::Allow, "Simulation passed and governance cleared.".to_string()),
                    GAction::SoftBlock => (Action::SoftBlock, verdict.reason),
                    GAction::HardBlock => (Action::HardBlock, verdict.reason),
                    GAction::AuditRequired => (Action::AuditRequired, verdict.reason),
                }
            },
            SimulationVerdict::Failure(msg) => (Action::HardBlock, format!("Dry-run simulation failed: {}", msg)),
            SimulationVerdict::CausalAlert(msg) => (Action::AuditRequired, format!("CAUSAL ALERT: {}", msg)),
        };

        Ok(Response::new(ToolCallVerdict {
            action: action as i32,
            reason,
        }))
    }

    async fn commit_tool_call(
        &self,
        request: Request<ToolOutcomeRequest>,
    ) -> Result<Response<CommitAck>, Status> {
        let req = request.into_inner();
        info!("Committing tool call outcome: {}", req.tool_call_id);
        
        let mut ledger = self.ledger.write().await;
        let _ = ledger.append_event("TOOL_OUTCOME", req.outcome_json.as_bytes());

        Ok(Response::new(CommitAck { committed: true }))
    }

    async fn get_causal_trace(
        &self,
        request: Request<TraceRequest>,
    ) -> Result<Response<TraceResponse>, Status> {
        let req = request.into_inner();
        let mut ledger = self.ledger.write().await;
        
        let plan_hash = if req.plan_hash.is_empty() { None } else { Some(req.plan_hash.as_str()) };
        let steps = self.trace_engine.reconstruct_trace(&mut ledger, plan_hash);
        
        Ok(Response::new(TraceResponse {
            events: steps.into_iter().map(|s| CausalEvent {
                timestamp: s.timestamp,
                event_type: s.event_type,
                payload: s.payload,
            }).collect(),
        }))
    }

    type SemanticHeartbeatStream = tokio_stream::wrappers::ReceiverStream<Result<KernelSignal, Status>>;

    async fn semantic_heartbeat(
        &self,
        _request: Request<tonic::Streaming<HeartbeatSignal>>,
    ) -> Result<Response<Self::SemanticHeartbeatStream>, Status> {
        Err(Status::unimplemented("Heartbeat stream not yet active"))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let current_dir = env::current_dir()?;
    let ledger_path = current_dir.join("causal_ledger.bin");
    info!("CausalOS Sidecar Kernel starting... (Ledger: {:?})", ledger_path);

    let ledger = Arc::new(RwLock::new(CausalLedger::open(&ledger_path)?));
    let ranker = Arc::new(RwLock::new(Ranker::new(42)));

    let kernel_host = KernelHost {
        governance: GovernanceEngine::new(),
        diagnostics: DiagnosticEngine::new(),
        promotion: PromotionManager::new(),
        trace_engine: TraceEngine::new(),
        simulator: HybridSimulator::new(),
        ranker,
        ledger,
    };

    let pipe_task = tokio::spawn(async move {
        if let Err(e) = run_named_pipe_server("causalos-kernel").await {
            eprintln!("Named Pipe server error: {:?}", e);
        }
    });

    let addr = "[::1]:50051".parse()?;
    info!("Control Plane listening on {}", addr);
    
    let grpc_task = tokio::spawn(async move {
        if let Err(e) = Server::builder()
            .add_service(KernelServiceServer::new(kernel_host))
            .serve(addr)
            .await {
                eprintln!("gRPC server error: {:?}", e);
            }
    });

    info!("Sidecar is ready.");

    tokio::select! {
        _ = pipe_task => info!("Named pipe task finished"),
        _ = grpc_task => info!("gRPC task finished"),
    }

    Ok(())
}

