# Design Specifications: Causal Memory & Governance

## 1. Hierarchical Causal Memory

### 1.1 Memory Partitioning (L1)
The L1 cache in the Data Plane must be partitioned by "Step-Class" to prevent context pollution between different phases of agent execution.
- **Planning Segment:** Stores high-level strategies and failure modes from previous plan formations.
- **Tool Segment:** Dedicated caches for specific tool behaviors (e.g., `git-commit-patterns`, `db-query-failures`).
- **Post-Tool Segment:** Context for interpreting results and detecting "silent" errors.

### 1.2 Hybrid Retrieval Logic (L2/L3)
Retrieval from Section 2/3 must be a hybrid scoring function:
`Score = (w_s * SemanticSim) + (w_sym * SymbolicMatch) + (w_r * Recency) + (w_c * Confidence) - (w_d * Decay)`
- **Symbolic Filters:** Strict matches on `tool_version`, `environment`, and `project_id`.
- **Confidence Decay:** Confidence scores are penalized if a pattern is injected but the outcome results in a misprediction.

### 1.3 Determinism & Replay
Every injection and governance decision must be deterministic for auditability.
- **Snapshots:** Every memory segment is identified by a `snapshot_hash` (content-addressed).
- **Ranking Seeds:** The ranking function must use a seeded randomizer or fixed sorting to ensure identical inputs + same memory snapshot = identical output.

---

## 2. Governance Loop & Control

### 2.1 Pre-flight: The Plan Contract
Before an Agent starts execution, the Kernel generates a **Plan Contract**:
- **Risk Score:** 0 (Low) to 1 (Critical).
- **Required Invariants:** List of hard constraints that must hold during this specific plan.
- **Watchpoints:** Step-specific triggers (e.g., "Pause if arguments to `rm` contains `*`").
- **Prefetch Index:** Ranked list of L2/L3 patterns to be pre-loaded into L1.

### 2.2 In-flight: Two-Phase Commit
All high-impact tool calls follow a strict 2PC-style atomic flow:
1. **Prepare (Sidecar):** Kernel validates call against authoritative invariants and reserves an idempotency key.
2. **Execute (Agent):** The tool call is performed in the Data Plane.
3. **Commit (Sidecar):** Final outcome is recorded to the Causal Ledger.

### 2.3 Safe Preemption (Interruption)
The Kernel must support **Interruption Semantics**:
- Every tool wrapper must support a `cancel_token`.
- The Sidecar can issue an `ABORT_STEP` or `HALT_EXECUTION` signal at any point during the loop.

---

## 3. Causal Promotion Pipeline (L2 → L3)
To share intelligence across the organization safely:
1. **Sanitization:** Strip PII, Secrets, and Internal IDs using a regex-first + LLM-deferred pipeline.
2. **Generalization:** Convert specific file paths (`/home/user/repo/A`) to generalized patterns (`REPOROOT/A`).
3. **Statistical Validation:** A pattern must have an $N$-frequency success/failure correlation across at least 2 different environments before promotion to L3 CNS.
