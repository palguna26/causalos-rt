# Implementation Roadmap: 4-Week Alpha Sprint

## Phase 1: Core Architecture & Control Plane (Week 1)
**Goal:** Establish the Sidecar Kernel and binary IPC foundation.
- **M1:** Build the Rust Sidecar boilerplate with an async event-loop.
- **M2:** Implement the **Append-Only Causal Ledger** using Protocol Buffers.
- **M3:** Establish **Binary IPC over UDS** for basic heartbeat and log transmission.
- **M4:** Build the **Deterministic Seeded Ranking** boilerplate for L2 retrieval.

## Phase 2: Data Plane & Embedded Memory (Week 2)
**Goal:** Implement the L1 cache, Shared Memory, and the Guest SDK.
- **M1:** Build the **Embedded Data Plane (Rust + WASM)** SDK.
- **M2:** Implement the **Read-only versioned Shared Memory** segment manager.
- **M3:** Implement the **L1 Partitioned Cache** (Planning/Tool/Error).
- **M4:** Build the **Prefetch Channel** for speculative memory ingestion.

## Phase 3: Governance & Live Enforcement (Week 3)
**Goal:** Implement Plan Contracts and authoritative tool-call interception.
- **M1:** Implement the **Plan Contract Generator** in the Sidecar (Risk Score + Invariants).
- **M2:** Implement **Watchpoint Triggers** in the Data Plane.
- **M3:** Implement the **Two-Phase Commit (2PC)** flow for tool calls.
- **M4:** Build the **Safe Preemption (Abort)** mechanism for tools.

## Phase 4: Tiered Diagnostics & CNS Promotion (Week 4)
**Goal:** Close the loop with learning and global intelligence distribution.
- **M1:** Build the **Tier 1 (Deterministic) & Tier 2 (Heuristic) Diagnostic** parsers.
- **M2:** Implement the **Negative Reinforcement (Penalty)** logic for mispredictions.
- **M3:** Build the **L3 Sanitization & Promotion Pipeline** (PII stripping).
- **M4:** Finalize the **"Causal Trace" Dashboard** (Otel integration).

---

## Technical Gates for Approval
1. **Gate 1 (End of Week 1):** Successful "Hello World" agent-to-sidecar log recording.
2. **Gate 2 (End of Week 2):** p99 latency for L1 injection < 20ms in mock stress tests.
3. **Gate 3 (End of Week 3):** Confirmed block of invariant-violating tool call with 0 bypass.
4. **Gate 4 (End of Week 4):** Successful promotion of a localized L2 pattern to global L3 after sanitization.
