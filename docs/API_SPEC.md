# API & Protocol Specification (Alpha)

This document defines the interface between Agent frameworks and the CausalOS Agent Runtime.

## 1. Control Plane (gRPC / Port 50051)

The Control Plane manages high-latency, safety-critical operations.

### Service: `KernelService`

```protobuf
service KernelService {
    // 1. Planning & Contracts
    rpc EvaluatePlan(PlanRequest) returns (PlanContract);
    rpc RecordOutcome(OutcomeRequest) returns (OutcomeResponse);
    
    // 2. Tool Governance
    rpc PrepareToolCall(ToolCallRequest) returns (ToolCallVerdict);
    rpc CommitToolCall(ToolOutcomeRequest) returns (CommitAck);
    
    // 3. Institutional Trace
    rpc GetCausalTrace(TraceRequest) returns (TraceResponse);
    
    // 4. Heartbeat (Future)
    rpc SemanticHeartbeat(stream HeartbeatSignal) returns (stream KernelSignal);
}
```

### Key Message: `PlanContract`
Generated upon plan evaluation. Contains the `contract_hash` which must be linked to all subsequent tool actions.
- `risk_score`: Heuristic score (0.0 to 1.0) indicating plan danger.
- `required_invariants`: List of checks the Kernel will enforce.

---

## 2. In-Flight Hot Path (Named Pipes / FatBuffers)

The Hot Path handles ultra-low latency signals.

### Transport: `\\.\pipe\causalos-kernel` (Windows)

#### Injection Request (FlatBuffers)
Used by the SDK to request immediate L1 context.
```fbs
table ContextInjectionRequest {
  current_step_id: string;
  local_context_snapshot: string;
}
```

#### Shared Memory L1 (Zero-Copy)
The L1 segment is mapped directly into the Agent process memory for O(1) retrieval of causal patterns.
- **Header**: `CausalEpochHeader` (8-byte aligned)
- **Data**: `L1Partition` (contains prioritized prompt snippets)

---

## 3. Causal Ledger Format (Binary)

The `causal_ledger.bin` is an append-only binary log.

| Segment | Offset | Size | Purpose |
| :--- | :--- | :--- | :--- |
| **Header** | 0 | 88 bytes | Epoch state, checksums, and metadata. |
| **Body** | 88 | Variable | Series of `CausalEvent` entries (Intent + Sanitized Outcome). |

## 4. Signal Hierarchy
1.  **ALLOW**: Standard execution path.
2.  **SOFT_BLOCK**: Agent is redirected to a safer alternative tool.
3.  **HARD_BLOCK**: Execution is halted; contract is voided.
4.  **AUDIT_REQUIRED**: Execution pauses for human-in-the-loop verification.
