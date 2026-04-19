# Architecture Specification: CausalOS Agent Runtime

## 1. Overview
CausalOS utilizes a **Split-Plane Architecture** to isolate safety-critical governance and institutional memory management from the high-frequency execution loop of the AI agent.

## 2. Component breakdown

### 2.1 Control Plane (The Sidecar Kernel)
The Sidecar is the "Source of Truth" implemented in Rust.
- **Service Layer (tonic/gRPC)**: Exposes the Kernel API for contracts and traces.
- **Governance Engine**: Evaluates plans against heuristic guardrails.
- **Diagnostic Engine**: Determines "Deterministic Success" vs "Causal Failure".
- **Promotion Manager**: Sanitizes and promotes successful execution patterns to the ledger.
- **Causal Ledger**: Binary append-only persistence of institutional events.

### 2.2 Data Plane (Embedded Hot Path)
A lightweight SDK integrated into the Agent's reasoning path.
- **Shared Memory (L1)**: Maps the Kernel's hot patterns directly into the Agent's address space.
- **Named Pipe Client**: Async communication with the Kernel watchdog for heartbeats and injections.

## 3. Communication Fabric (IPC)

| Channel | Layer | Strategy | Performance Target |
| :--- | :--- | :--- | :--- |
| **Control Plane** | Governance | gRPC over Loopback | <50ms (Plan Analysis) |
| **Hot Path** | Signaling | Windows Named Pipes | <5ms (Validity Signals) |
| **Cache Layer** | Context | Win32 Shared Memory | <1ms (Zero-copy Read) |

## 4. The Learning Loop (CNS)
CausalOS implements a **Closed-loop Neuro-Symbolic (CNS)** system:
1.  **Intent Evaluation**: Agent proposes a plan. Kernel grants a contract.
2.  **Instrumented Execution**: Agent performs tools calls. Kernel enforces invariants.
3.  **Outcome Classification**: Kernel analyzes the "Details" segment of the outcome.
4.  **Reinforcement**: Ranker adjusts weights based on success/failure patterns.
5.  **Promotion**: High-confidence successes are masked (PII removal) and written to the ledger as new "Causal Context".

## 5. Resilience & Survival Mode
- **Process Isolation**: The Kernel runs independently. If the Agent process crashes, the Kernel preserves the trace up to the last ACK.
- **Deterministic Guardrails**: Even if the Sidecar logic fails, the "Hard Block" default ensures the system fails safe.
