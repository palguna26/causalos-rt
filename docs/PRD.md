# Product Requirements Document (PRD): CausalOS v2

## 1. Product Vision
Deliver a persistent, deterministic "Governance Layer" for AI Agents that eliminates regressions and provides a structured "Safety Harness" for high-stakes enterprise operations.

## 2. The Core Moat: "Causal Memory"
Unlike traditional logs, CausalOS builds a **Directed Acyclic Graph (DAG)** of agent decisions. 
- **Fact**: The system can detect when an agent deviates from a historically successful trajectory.
- **Fact**: Every tool execution is gated by a **Hybrid Simulation** that combines dry-runs with causal analysis.

## 3. Targeted User Segments
- **FinOps Innovations Inc. (Pilot Case)**: Cloud cost optimization agents requiring 100% safety on production infrastructure changes.
- **Enterprise DevOps**: Agents managing CI/CD pipelines where state-drift makes traditional "Simulation" unreliable.

## 4. Key Functional Requirements (v2 Implemented)

### 2-Phase Commit (2PC) Tool Governance
- **Prepare Phase**: Kernel executes a sub-process probe and checks history.
- **Commit Phase**: Kernel records final outcome and updates the trajectory graph.

### Trajectory-Based RCA
- Automated detection of the "Divergence Point" in failures.
- Classification of outcomes into `DeterministicSuccess` or `CausalFailure`.

## 5. Non-Functional Requirements
- **Performance**: gRPC latency < 10ms for pattern matching.
- **Reliability**: Fail-safe defaults for simulation timeouts.
- **Accuracy**: 100% factual documentation with zero legacy assumptions.

---

## 6. Project Scope Constraints (Strict Implementation)
- **Causal Ledger**: Binary persistence enabled.
- **Named Pipes**: DEPRECATED in favor of gRPC.
- **Shared Memory**: REMOVED to maintain factual consistency with current kernel logic.
