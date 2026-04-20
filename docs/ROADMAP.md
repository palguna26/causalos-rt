# Roadmap: CausalOS Agent Runtime

## Phase 1: Research & Kernel Foundation (COMPLETED)
- [x] Multi-agent Split-plane Architecture design.
- [x] Initial binary ledger implementation.
- [x] Basic gRPC service layer.

## Phase 2: Causal Governance (v2 - CURRENT)
- [x] **Causal Ledger DAG**: Transitioned for linear logs to a Directed Acyclic Graph (linked via `parent_hash`).
- [x] **Two-Phase Commit (2PC)**: Implemented `PrepareToolCall` and `CommitToolCall`.
- [x] **Hybrid Simulation Engine**: Native probes + Causal History filters.
- [x] **Trajectory RCA**: Automated failure divergence detection.
- [x] **Documentation Hardening**: Stripped all "Future/Alpha" assumptions to deliver 100% factual specifications.

## Phase 3: Pilot Readiness (NEXT)
- [ ] **Cross-Platform Integration**: Support for Linux control groups and Docker state-snapping in simulations.
- [ ] **Institutional Dashboard**: Visualizing the Causal Trace for human audits.
- [ ] **Advanced CNS Promotion**: Automated scrubbing of PII from causal memory (Sanitization logic).

## Phase 4: Scaling & Security (FUTURE)
- [ ] **Sovereign Memory**: Zero-knowledge proof sharing of causal failures across different enterprise instances.
- [ ] **Hot-Path Re-integration**: Re-introducing ultra-low latency signaling once the core gRPC bus is stabilized.
