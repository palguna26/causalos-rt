# API & Protocol Specification (Alpha)

This document defines the interface between requested Agent frameworks and the CausalOS Agent Runtime.

## 1. Kernel Service (gRPC / UDS)

```protobuf
service KernelService {
    // 1. Lifecycle & Planning
    rpc EvaluatePlan(PlanRequest) returns (PlanContract);
    rpc RecordOutcome(OutcomeRequest) returns (OutcomeResponse);
    
    // 2. Governance
    rpc PrepareToolCall(ToolCallRequest) returns (ToolCallVerdict);
    rpc CommitToolCall(ToolOutcomeRequest) returns (CommitAck);
    
    // 3. Heartbeat
    rpc SemanticHeartbeat(stream HeartbeatSignal) returns (stream KernelSignal);
}

message PlanRequest {
    string agent_id = 1;
    string project_id = 2;
    string plan_text = 3;
    repeated string intent_tags = 4;
}

message PlanContract {
    string contract_hash = 1;
    float risk_score = 2;
    repeated Invariant required_invariants = 3;
    repeated Watchpoint watchpoints = 4;
}
```

## 2. In-Flight Hot Path (Binary IPC / Protobuf)

```protobuf
message ContextInjectionRequest {
    string current_step_id = 1;
    string local_context_snapshot = 2;
}

message InvariantCheck {
    string tool_name = 1;
    map<string, string> arguments = 2;
}

message InvariantVerdict {
    enum Action {
        ALLOW = 0;
        SOFT_BLOCK = 1;
        HARD_BLOCK = 2;
        AUDIT_REQUIRED = 3;
    }
    Action action = 1;
    string reason = 2;
}
```

## 3. Shared Memory Snapshot Structure (C Encoding)

```c
struct CausalEpochHeader {
    uint64_t epoch_id;
    uint8_t snapshot_hash[32];
    uint32_t pattern_count;
    uint32_t segment_size;
}

struct CausalPatternEntry {
    uint32_t pattern_id;
    float confidence_score;
    uint32_t embedding_offset; // Offset into the blobs segment
    uint32_t metadata_offset;  // Offset into the strings segment
}
```

---

## 4. Signal Definitions
- `HALT_EXECUTION`: Immediate stop signal from Sidecar to Agent.
- `PREFETCH_SYNC`: Signal from Sidecar containing the next Epoch for Data Plane.
- `STALL_DETECTED`: Internally triggered signal when heartbeat semantic state hasn't changed.
