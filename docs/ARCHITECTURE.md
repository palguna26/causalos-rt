# Architecture Specification: CausalOS Agent Runtime

## 1. Overview
The CausalOS Agent Runtime utilizes a **Split-Plane Architecture** to isolate high-stakes governance and large-scale memory management from the high-frequency execution path of the agent.

## 2. Component Breakdown

### 2.1 Control Plane (The Sidecar Kernel)
The Sidecar is the "Source of Truth" for the entire system.
- **Language:** Rust
- **Runtime:** Independent process (e.g., systemd service, K8s sidecar).
- **Core Modules:**
    - **Causal Ledger:** Manages the append-only, event-sourced log of all actions and outcomes.
    - **Policy Engine:** Evaluates Plan Contracts and enforces Hard Invariants.
    - **Memory Coordinator:** Handles L2/L3 retrieval and sanitization.
    - **Watchdog:** Monitors Agent heartbeats and classifies failure modes (stall, loop, crash).

### 2.2 Data Plane (The Embedded Fast Path)
The Data Plane is a lightweight SDK integrated directly into the Agent's reasoning loop.
- **Languages:** Rust (Core), with bindings for Python, TypeScript, and Go.
- **Core Modules:**
    - **L1 Cache Manager:** Handles ultra-low latency context injection (<10ms).
    - **Preliminary Guardrails:** Soft-blocks obvious policy violations locally.
    - **IPC Client:** Manages the binary and gRPC communication with the Sidecar.

## 3. Communication Fabric (IPC)

| Channel | Layer | Strategy | Purpose |
| :--- | :--- | :--- | :--- |
| **Hot Path** | Transport | Binary over UDS | Injections, Validations, Heartbeats. |
| **Control Path**| Application | gRPC over UDS | Plan creation, Config, Admin. |
| **Memory Path** | Storage | Versioned Shared Memory | Read-only L1/L2 pattern ingestion. |
| **Prefetch** | Async Push | Push Stream | Warming L1 before tool execution. |

### 3.1 Shared Memory Design
- **Sole Writer:** Only the Sidecar can write to the shared memory segment.
- **Epoch Versioning:** Memory is divided into versioned segments identified by `epoch_id` and `snapshot_hash`.
- **Atomic Switching:** The Data Plane switches between segments atomically via pointer updates.

## 4. Resilience & Survival Model
- **Process Isolation:** The Sidecar is a parent/sidecar process. If the Agent process dies, the Sidecar remains active.
- **Persistence:** All state transitions are written to an append-only log before being ACKed, ensuring "At-least-once" delivery of causal signals.
- **Degradation:** If the Sidecar is overloaded, the Data Plane is designed to fail-safe into "L1-only" mode with elevated logging.

## 5. Security & Isolation
- **Multi-Tenancy:** L2 namespaces are logically isolated via row-level security or unique encryption keys per project.
- **Zero-Bypass:** All tool calls *must* be wrapped by the Data Plane SDK. No direct Path to execution is permitted in production environments.
