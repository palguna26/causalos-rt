# API & Protocol Specification: CausalOS v2

This document defines the deterministic interface between AI Agent frameworks and the CausalOS Kernel.

## 1. Governance Protocol (gRPC)
CausalOS enforces safety through a **Two-Phase Commit (2PC)** protocol. All tool calls must be cleared by the Kernel before they are committed to external environments.

### Service: `KernelService`
Located at `port 50051` (Localhost).

```protobuf
service KernelService {
    // Phase 1: Hybrid Simulation (Probe + Causal Guard)
    rpc PrepareToolCall(ToolCallRequest) returns (ToolCallVerdict);
    
    // Phase 2: Final Persistence & Learning
    rpc CommitToolCall(ToolOutcomeRequest) returns (CommitAck);
    
    // Auxiliary: Planning & Trace
    rpc EvaluatePlan(PlanRequest) returns (PlanContract);
    rpc GetCausalTrace(TraceRequest) returns (TraceResponse);
}
```

---

## 2. Message Structures (Facts)

### 2.1 `PrepareToolCall`
The "Gatekeeper" call. Triggers a `HybridSimulator` run.
- **Input**: `tool_name`, `arguments` (JSON), `contract_hash`.
- **Verdict Hierarchy**:
    1.  **ALLOW**: Simulation passed + No historical failures found.
    2.  **AUDIT_REQUIRED**: Simulation passed, but historical data shows similar paths failed 2+ times.
    3.  **SOFT_BLOCK**: Simulation failed, but Kernel suggests a repair (e.g. "Add --yes flag").
    4.  **HARD_BLOCK**: Critically unsafe (e.g. credential deletion in production).

### 2.2 `CommitToolCall`
The "Persistence" call. Writes the result to the **Causal Ledger DAG**.
- **Input**: `tool_id`, `success` (bool), `output_details` (String).
- **Processing**: The Kernel maps the output to an `OutcomeClass` and updates the trajectory state.

---

## 3. Communication Strategy
CausalOS v2 utilizes standard **HTTP/2 based gRPC**. 
- **Latency Target**: <10ms for `Prepare` (Cache Hit) to <100ms (Cold Probe/Subprocess).
- **Security**: The kernel binds exclusively to `127.0.0.1` by default to prevent network-level exposure of the governance layer.

---

## 4. Signal Definitions
- **Divergence RCA**: Triggered during `CommitToolCall` if a failure occurs on a trajectory that was previously successful. The Kernel returns a `TrajectoryMismatch` error with the previous successful parent hash.
