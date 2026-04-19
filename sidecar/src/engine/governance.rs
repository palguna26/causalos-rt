use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

pub struct ToolCallVerdict {
    pub action: Action,
    pub reason: String,
}

pub enum Action {
    Allow,
    SoftBlock,
    HardBlock,
    AuditRequired,
}

pub struct GovernanceEngine {
    // Failure density map: tool_name -> failure_count
    failure_density: HashMap<String, u32>,
}

impl GovernanceEngine {
    pub fn new() -> Self {
        Self {
            failure_density: HashMap::new(),
        }
    }

    pub fn evaluate_plan(&self, plan_text: &str) -> (f32, Vec<String>) {
        let mut risk_score: f32 = 0.0;
        let mut invariants = Vec::new();

        // Heuristic: Check for dangerous keywords
        if plan_text.contains("delete") || plan_text.contains("remove") || plan_text.contains("rm ") {
            risk_score += 0.4;
            invariants.push("FILE_PURGE_SAFETY".to_string());
        }

        if plan_text.contains("sudo") || plan_text.contains("admin") || plan_text.contains("chmod") {
            risk_score += 0.5;
            invariants.push("PRIVILEGE_ESCALATION_CHECK".to_string());
        }

        (risk_score.min(1.0), invariants)
    }

    pub fn evaluate_tool_call(&self, tool_name: &str, arguments_json: &str) -> ToolCallVerdict {
        // Critical block: recursive deletes on root or sensitive paths
        if tool_name == "run_command" && (arguments_json.contains("rm -rf /") || arguments_json.contains("rm -rf C:\\")) {
            return ToolCallVerdict {
                action: Action::HardBlock,
                reason: "Attempted recursive delete on system root".to_string(),
            };
        }

        // Higher density failures -> Audit Required
        let failure_count = self.failure_density.get(tool_name).unwrap_or(&0);
        if *failure_count > 5 {
            return ToolCallVerdict {
                action: Action::AuditRequired,
                reason: format!("Tool {} has high failure density ({} recent failures)", tool_name, failure_count),
            };
        }

        ToolCallVerdict {
            action: Action::Allow,
            reason: "Call conforms to safety heuristics".to_string(),
        }
    }

    pub fn record_failure(&mut self, tool_name: &str) {
        *self.failure_density.entry(tool_name.to_string()).or_insert(0) += 1;
    }
}
