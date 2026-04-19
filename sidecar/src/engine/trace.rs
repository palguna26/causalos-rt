use crate::storage::ledger::CausalLedger;
use tracing::info;

#[derive(Debug, serde::Serialize)]
pub struct CausalStep {
    pub timestamp: u64,
    pub event_type: String,
    pub payload: String,
}

pub struct TraceEngine;

impl TraceEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn reconstruct_trace(&self, ledger: &mut CausalLedger, _plan_hash: Option<&str>) -> Vec<CausalStep> {
        info!("Reconstructing causal trace from ledger...");
        let mut steps = Vec::new();
        
        // In this Alpha, we just read the last 5 events
        // In production, we would filter by plan_hash
        if let Ok(entries) = ledger.read_last_n(10) {
            for (header, data) in entries {
                steps.push(CausalStep {
                    timestamp: header.timestamp,
                    event_type: String::from_utf8_lossy(&header.event_type).to_string(),
                    payload: String::from_utf8_lossy(&data).to_string(),
                });
            }
        }
        
        steps
    }
}
