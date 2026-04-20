use std::process::Command;
use tracing::{info, warn};
use crate::storage::ledger::CausalLedger;
use serde_json::Value;

pub enum SimulationVerdict {
    Success,
    Failure(String),
    CausalAlert(String),
}

pub trait DiagnosticProbe: Send + Sync {
    fn tool_name(&self) -> &str;
    fn run_simulation(&self, args: &Value) -> SimulationVerdict;
}

pub struct ShellProbe {
    pub tool: String,
    pub dry_run_flag: String,
}

impl DiagnosticProbe for ShellProbe {
    fn tool_name(&self) -> &str {
        &self.tool
    }

    fn run_simulation(&self, _args: &Value) -> SimulationVerdict {
        info!("Running external simulation for {}: {} {}", self.tool, self.tool, self.dry_run_flag);
        
        let output = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args(["-Command", &format!("{} --help", self.tool)]) // Using --help as a generic probe if dry-run doesn't exist
                .output()
        } else {
            Command::new(&self.tool)
                .arg(&self.dry_run_flag)
                .output()
        };

        match output {
            Ok(out) if out.status.success() => SimulationVerdict::Success,
            Ok(out) => SimulationVerdict::Failure(String::from_utf8_lossy(&out.stderr).to_string()),
            Err(e) => SimulationVerdict::Failure(e.to_string()),
        }
    }
}

pub struct HybridSimulator {
    probes: Vec<Box<dyn DiagnosticProbe>>,
}

impl HybridSimulator {
    pub fn new() -> Self {
        Self {
            probes: vec![
                Box::new(ShellProbe { tool: "terraform".to_string(), dry_run_flag: "plan".to_string() }),
                Box::new(ShellProbe { tool: "npm".to_string(), dry_run_flag: "install --dry-run".to_string() }),
            ],
        }
    }

    pub fn simulate(&self, tool_name: &str, args: &Value, ledger: &CausalLedger) -> SimulationVerdict {
        // 1. External Probe
        let probe_result = if let Some(probe) = self.probes.iter().find(|p| p.tool_name() == tool_name) {
            probe.run_simulation(args)
        } else {
            SimulationVerdict::Success // No probe, assume valid for now
        };

        // 2. Causal Filter (The Moat)
        if let SimulationVerdict::Success = probe_result {
            info!("External probe passed. Checking Causal History...");
            // Logic: Scan ledger for past failures of this tool in the current project context
            // In a real implementation, we would hash the current state and search for similar contexts
            if let Ok(recent) = ledger.read_last_n(50) {
                for (header, data) in recent {
                    let type_str = String::from_utf8_lossy(&header.event_type);
                    if type_str == "OUTCOME" {
                        let outcome: Value = serde_json::from_slice(&data).unwrap_or_default();
                        if outcome["tool_name"] == tool_name && outcome["success"] == false {
                            warn!("Causal Alert: External probe passed, but history shows past failure for {}!", tool_name);
                            return SimulationVerdict::CausalAlert(format!("Past failure detected for {} despite successful dry-run.", tool_name));
                        }
                    }
                }
            }
        }

        probe_result
    }
}
