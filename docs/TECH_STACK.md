# Technology Stack: CausalOS Agent Runtime (v2)

CausalOS is built for high-reliability, low-latency, and deterministic auditability in high-stakes agentic environments.

## 1. Programming Languages
- **Sidecar Kernel:** Rust (1.75+) — Provides memory safety and deterministic resource control.
- **Protocol:** Protobuf / gRPC (v3).

## 2. Core Dependencies

### Communication & RPC
| Library | Role | Specification |
| :--- | :--- | :--- |
| **tonic** | gRPC Runtime | Handles the 2PC (Two-Phase Commit) control plane. |
| **prost** | Protobuf Engine | Synchronized with `kernel.proto` for stable API contracts. |

### System & Intelligence
| Library | Role | Specification |
| :--- | :--- | :--- |
| **tokio** | Async Runtime | Manages concurrent gRPC streams and sub-process simulation probes. |
| **fxhash** | Fast Hashing | Optimized O(1) retrieval for Causal Trajectory identification. |
| **serde_json** | Data Serialization | Used for classifying tool outcomes and metadata parsing. |
| **tracing** | Metrics/Logs | Structured instrumentation for institutional auditability. |

## 3. Storage & Memory Model

### Causal Ledger (DAG)
The Ledger is a custom binary format implementing a **Directed Acyclic Graph**. 
Each event entry follows this factual byte-level specification:
- `[8 bytes]`: Timestamp (unix nanos)
- `[8 bytes]`: Parent Hash (links to previous event in trajectory)
- `[4 bytes]`: Type Length
- `[V bytes]`: Type (e.g., "PREPARE", "COMMIT", "OUTCOME")
- `[4 bytes]`: Data Length
- `[V bytes]`: JSON Data (The actual causal context)

### Intelligence Store
- **In-memory Index**: A global hash-map using `fxhash` for sub-millisecond lookup of historical failure patterns.
- **Trajectory RCA**: Divergence detection logic implemented in `engine/diagnostics.rs`.

## 4. Development & Execution tools
- **cargo**: Main build and dependency manager.
- **protoc**: Compiled via `tonic-build` during the build phase.
- **Subprocess Probes**: Kernel uses `std::process` to execute external dry-runs (e.g. `terraform plan`) as part of the Hybrid Simulation loop.
