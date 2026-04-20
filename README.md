# CausalOS: The Deterministic Governance Layer for AI Agents

CausalOS is a production-grade agent kernel designed to eliminate "Hallucinatory Regressions" in high-stakes environments. It transforms the AI agent's reasoning loop from a best-effort chat into a **Two-Phase Commit (2PC)** execution protocol backed by institutional causal memory.

## 🚀 The V2 Moat: Causal Memory

Unlike traditional logging, CausalOS builds a **Directed Acyclic Graph (DAG)** of every agent decision. This allows the kernel to perform **Trajectory-Aware Safety**:

- **Hybrid Simulation**: Every critical tool call (e.g. `terraform apply`) is gated by a simulation that combines **Native Probes** (Dry-runs) with **Causal Filters** (History).
- **Divergence RCA**: If a failure occurs, the kernel identifies the exact point where the current path deviated from a historically successful trajectory.

## 🏗 Architecture (Facts)

### 1. Control Plane (The Kernel)
Implemented in **Rust**, the Kernel acts as the "Source of Truth" for safety and memory.
- **Service API**: High-performance gRPC (v2) bus.
- **Governance**: Enforces a strict 2-Phase Commit (Prepare/Commit) flow.
- **Memory**: Custom binary DAG ledger recording every intent and outcome with parent-child links.

### 2. Implementation Specifications
- **Format**: `[Timestamp][ParentHash][TypeLen][Type][DataLen][JSON_Data]`
- **Engine**: Subprocess dry-run probes with `fxhash` trajectory lookup.

## 🛠 Getting Started

### Prerequisites
- Rust 1.75+
- Protobuf Compiler (`protoc`)

### Installation
```bash
git clone https://github.com/palguna26/causalos-runtime.git
cd causalos-runtime/sidecar
cargo build --release
```

### Protocol Usage
1. **Prepare**: Send `PrepareToolCall` to simulate the action.
2. **Execute**: Perform the tool action *only* if the verdict is `ALLOW`.
3. **Commit**: Record the outcome via `CommitToolCall` to update the Causal Ledger DAG.

## 🛡 Performance & Safety
- **Latency**: <10ms for pattern-aware governance checks.
- **Reliability**: Fail-safe defaults. If simulation probes time out, the action is `SOFT_BLOCK`-ed.

---
**CausalOS is built for the Pilot at FinOps Innovations Inc.**
