# Technology Stack: CausalOS Agent Runtime

CausalOS is built for high-reliability, low-latency, and deterministic auditability.

## 1. Programming Languages
- **Sidecar Kernel:** Rust (1.75+) — Chosen for memory safety, performance, and deterministic resource control.
- **SDK:** Rust (Core) with Cross-language Bindings (Future).
- **Protocol:** Protobuf / gRPC & FlatBuffers.

## 2. Frameworks & Libraries (Core)

### Communication & RPC
| Library | Role | Why? |
| :--- | :--- | :--- |
| **tonic** | gRPC (Control Plane) | Native Rust implementation of gRPC for the control path. |
| **prost** | Protobuf Compilation | High-performance Protobuf serialization for cross-process signals. |
| **flatbuffers** | Hot Path Serialization | Zero-copy serialization for the L1 cache injection path. |

### System & Async
| Library | Role | Why? |
| :--- | :--- | :--- |
| **tokio** | Async Runtime | Industry-standard async runtime for concurrent gRPC and Pipe handling. |
| **windows-sys** | Win32 API Access | Low-level access to Named Pipes and Shared Memory (CreateFileMapping). |
| **zerocopy** | Memory Alignment | Safe abstractions for zero-copy mapping of binary data from shared memory. |

### Observability
| Library | Role | Why? |
| :--- | :--- | :--- |
| **tracing** | Internal Logging | Structured diagnostics and trace classification. |
| **opentelemetry** | Distributed Tracing | Standards-compliant traces for institutional auditability. |

## 3. Storage & Memory Model
- **Causal Ledger:** Custom binary append-only format for event-sourced persistence.
- **L1 Cache:** Memory-mapped shared memory segment (NT Shared Memory Namespace).
- **Ranking Store:** In-memory FxHash map with weighted reinforcement weights.

## 4. Development Tools
- **protoc**: Protocol Buffers compiler.
- **cargo**: Build system and package manager.
- **Named Pipe Client (SDK)**: Specialized Windows named pipe implementation for hot-path signals.
