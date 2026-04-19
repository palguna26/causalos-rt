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
            // Heuristic check: Did the agent "simulate" success?
            if details.to_lowercase().contains("simulated") || details.to_lowercase().contains("dry run") {
                return OutcomeClass::HeuristicSuccess;
            }
            OutcomeClass::DeterministicSuccess
        } else {
            // Analyze failure reason
            if details.to_lowercase().contains("access denied") || details.to_lowercase().contains("permission") {
                return OutcomeClass::CausalFailure;
            }
            if details.to_lowercase().contains("not found") || details.to_lowercase().contains("404") {
                return OutcomeClass::CausalFailure;
            }
            OutcomeClass::CausalFailure
        }
    }
}
