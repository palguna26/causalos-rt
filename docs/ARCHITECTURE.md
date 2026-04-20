# Architecture Specification: CausalOS v2

CausalOS utilizes a **Causal Governance Architecture** to provide deterministic safety for autonomous agents in high-stakes environments.

## 1. The Kernel Host (Rust)
The Kernel is the primary gatekeeper of the system, implemented in safe Rust.

### 1.1 Governance Engine (The Watchdog)
Evolves tool-use from "Best Effort" to **Two-Phase Commit (2PC)**:
- **Phase 1 (Prepare)**: Orchestrates the `HybridSimulator`. 
- **Phase 2 (Commit)**: Finalizes the executive action and logs the causal outcome.

### 1.2 Intelligence Engine (The Memory)
Manages the **Causal Ledger DAG**. 
- **Trajectory Recognition**: Every session is a trajectory in the DAG.
- **Root Cause Analysis (RCA)**: Detects "Trajectory Divergence" by finding where a previously successful path (parent_hash) led to a new failure.

## 2. Hybrid Simulation Loop

CausalOS avoids the "Simulation Fallacy" (where a passing dry-run fails in production) by layering historical evidence over mechanical probes.

1.  **Probe Phase**: Executes a native dry-run (e.g., `terraform plan`) via subprocess.
2.  **Causal Filter Phase**: Cross-references the probe result against the `Intelligence Engine`.
3.  **Verdict**: Returns a deterministic signal (`ALLOW`, `AUDIT_REQUIRED`, etc.).

## 3. Storage Hierarchy (Factual)

| Component | Mechanism | Implementation |
| :--- | :--- | :--- |
| **Causal Ledger** | Binary DAG | `storage/ledger.rs` (linked via parent_hash) |
| **Pattern Index** | In-Memory Cache | `fxhash` map of historical outcomes |
| **Protocol** | Control Plane | `tonic` gRPC (HTTP/2) |

## 4. Resilience Patterns
- **Fail-Safe Default**: If the Simulation Probe fails to return within its timeout, the Kernel defaults to `SOFT_BLOCK`.
- **Causal Consistency**: The ledger prevents "Rewriting History" by enforcing an append-only DAG structure with cryptographic hashing (planned upgrade).

---

## 5. Eliminated Components (Removed for Reliability)
To ensure 100% factual accuracy, the following "Alpha" components have been stripped until completion:
- **L1 Shared Memory**: Removed legacy support to focus on gRPC performance.
- **Named Pipe Signaling**: Consolidating all signals into the unified gRPC control bus.
