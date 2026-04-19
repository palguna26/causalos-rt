use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct MemoryPattern {
    pub id: String,
    pub content: String,
    pub success_rate: f32,
    pub project_id: String,
}

pub struct Ranker {
    seed: u64,
    // Transient pattern state (would be persisted in a DB in production)
    pub patterns: std::collections::HashMap<String, MemoryPattern>,
}

impl Ranker {
    pub fn new(seed: u64) -> Self {
        Self { 
            seed,
            patterns: std::collections::HashMap::new(),
        }
    }

    pub fn apply_reinforcement(&mut self, pattern_id: &str, delta: f32) {
        if let Some(p) = self.patterns.get_mut(pattern_id) {
            p.success_rate = (p.success_rate + delta).clamp(0.0, 1.0);
        }
    }

    pub fn rank_patterns(&self, query: &str, current_project: &str, _candidates_ignored: Vec<MemoryPattern>) -> Vec<(String, f32)> {
        // Use internal patterns instead of external candidates for now to show reinforcement
        let mut scored: Vec<(String, f32)> = self.patterns.values()
            .map(|p| {
                let score = self.score_pattern(query, current_project, &p);
                (p.id.clone(), score)
            })
            .collect();

        // Deterministic sort: use score, then ID (to break ties)
        scored.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(&b.0))
        });

        scored
    }

    fn score_pattern(&self, query: &str, current_project: &str, pattern: &MemoryPattern) -> f32 {
        // 1. Semantic Similarity (Dummy for now, use token overlap)
        let query_tokens: std::collections::HashSet<&str> = query.split_whitespace().collect();
        let pattern_tokens: std::collections::HashSet<&str> = pattern.content.split_whitespace().collect();
        let intersection = query_tokens.intersection(&pattern_tokens).count() as f32;
        let similarity = intersection / (query_tokens.len().max(1) as f32);

        // 2. Success Rate
        let success_rate = pattern.success_rate;

        // 3. Project Locality
        let locality = if pattern.project_id == current_project { 1.0 } else { 0.2 };

        // Hybrid Score: (0.4 * Similarity) + (0.4 * SuccessRate) + (0.2 * Locality)
        let final_score = (0.4 * similarity) + (0.4 * success_rate) + (0.2 * locality);

        // Deterministic Jitter (Seeded)
        // This ensures that for the same seed/query, the ranking is absolutely stable
        let mut hasher = DefaultHasher::new();
        pattern.id.hash(&mut hasher);
        self.seed.hash(&mut hasher);
        let jitter = (hasher.finish() % 1000) as f32 / 10000.0; // max 0.1 jitter

        final_score + jitter
    }
}
