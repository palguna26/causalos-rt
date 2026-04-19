use causalos_sdk::ipc::client::SidecarClient;
use causalos_sdk::kernel_proto::{OutcomeRequest, PlanRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = SidecarClient::new("causalos-kernel");
    client.connect_control_plane("http://[::1]:50051").await.map_err(|e| anyhow::anyhow!(e))?;
    
    println!("Step 1: Evaluating a new plan...");
    let plan = PlanRequest {
        agent_id: "test-agent-001".to_string(),
        project_id: "causalos-core".to_string(),
        plan_text: "Refactor the IPC module to use Shared Memory for hot paths.".to_string(),
        intent_tags: vec!["refactor".to_string(), "ipc".to_string()],
    };
    
    let contract = client.evaluate_plan(plan).await.map_err(|e| anyhow::anyhow!(e))?;
    println!("Plan evaluated. Risk Score: {}", contract.risk_score);

    println!("\nStep 2: Recording SUCCESSFUL outcome (Promotion Test)...");
    let success_req = OutcomeRequest {
        plan_hash: "abcd-1234".to_string(),
        success_criteria: "IPC module refactored and tests passed.".to_string(),
        success: true,
        details: "Refactoring complete. Latency reduced from 50ms to 2ms. No regression detected.".to_string(),
    };
    
    let res = client.record_outcome(success_req).await.map_err(|e| anyhow::anyhow!(e))?;
    println!("Outcome recorded: recorded={}", res.recorded);

    println!("\nStep 3: Recording FAILURE outcome (Negative Reinforcement Test)...");
    let fail_req = OutcomeRequest {
        plan_hash: "fail-5678".to_string(),
        success_criteria: "Delete logs older than 7 days.".to_string(),
        success: false,
        details: "PERMISSION_DENIED: Access to /var/log/system.log is restricted.".to_string(),
    };
    
    let res = client.record_outcome(fail_req).await.map_err(|e| anyhow::anyhow!(e))?;
    println!("Failure recorded: recorded={}", res.recorded);

    println!("\nVerification Complete. Observe Sidecar logs for Promotion and Reinforcement marks.");
    Ok(())
}
