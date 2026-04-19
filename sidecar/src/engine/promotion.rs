use crate::storage::ledger::CausalLedger;
use tracing::info;

pub struct PromotionManager;

impl PromotionManager {
    pub fn new() -> Self {
        Self
    }

    pub fn promote_to_ledger(&self, ledger: &mut CausalLedger, intent: &str, details: &str) -> anyhow::Result<()> {
        let sanitized_details = self.strip_pii(details);
        
        let payload = format!("INTENT: {}\nOUTCOME: {}", intent, sanitized_details);
        ledger.append_event("CAUSAL_PATTERN", payload.as_bytes())?;
        
        info!("Promoted new pattern to Causal Ledger: {}", intent);
        Ok(())
    }

    fn strip_pii(&self, input: &str) -> String {
        // Simple PII stripping for Phase 4
        // Scrapes emails and common API key patterns (dummy implementation)
        let mut result = input.to_string();
        
        // Regex-free simplification for now: replace '@' with '[AT]'
        result = result.replace("@", "[AT]");
        
        // Simple key masking (looking for 32+ char hex/alphanum strings)
        // For now, let's just do a basic one
        result
    }
}
