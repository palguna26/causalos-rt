use crate::storage::ledger::CausalLedger;
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeClass {
    DeterministicSuccess,
    HeuristicSuccess,
    CausalFailure,
    Unknown,
}

pub struct DiagnosticEngine;

impl DiagnosticEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze_outcome(&self, details: &str, success: bool) -> OutcomeClass {
        if success {
            if details.to_lowercase().contains("simulated") || details.to_lowercase().contains("dry run") {
                return OutcomeClass::HeuristicSuccess;
            }
            OutcomeClass::DeterministicSuccess
        } else {
            OutcomeClass::CausalFailure
        }
    }

    pub fn perform_rca(&self, ledger: &CausalLedger, failed_hash: u64) -> Option<String> {
        info!("Performing Root Cause Analysis for event {:x}", failed_hash);
        
        // Logic: Find the failed event, trace back to its parent, 
        // and find the most recent successful sibling or ancestor that shares context.
        if let Ok(history) = ledger.read_last_n(100) {
            let mut failed_event = None;
            for (header, data) in &history {
                let current_hash = fxhash::hash64(data);
                if current_hash == failed_hash {
                    failed_event = Some(header);
                    break;
                }
            }

            if let Some(failed) = failed_event {
                // Search for a 'Success' event in the history that shares the same parent_hash
                // but has a different outcome.
                for (header, data) in &history {
                    if header.parent_hash == failed.parent_hash && header.parent_hash != 0 {
                        let type_str = String::from_utf8_lossy(&header.event_type);
                        if type_str == "OUTCOME" {
                           let outcome: serde_json::Value = serde_json::from_slice(&data).unwrap_or_default();
                           if outcome["success"] == true {
                               return Some(format!("Divergence detected: Path {:x} succeeded previously. Current failure likely due to state drift in project.", header.parent_hash));
                           }
                        }
                    }
                }
            }
        }
        
        Some("Root cause isolated: No previous success found for this trajectory. Check external dependencies.".to_string())
    }
}

