# Technology Stack & Tooling

## 1. Core Implementation
- **Kernel (Sidecar):** **Rust**. Chosen for zero-cost abstractions, memory safety, and deterministic execution without GC pauses.
- **Embedded SDK (Data Plane):** **Rust Core** with **WASM** modules for cross-platform integration and **PyO3/Node-API** bindings for language-specific ergonomics.

## 2. Communication & IPC
- **Binary Transport (Hot Path):** **FlatBuffers** (or Cap'n Proto). Optimized for zero-copy deserialization in performance-critical paths.
- **Control Interface:** **gRPC over UDS** (Unix Domain Sockets). Standardized, typed contracts for orchestration.
- **Shared Memory:** **Shared Memory (shm)** with `memfd` or platform-specific equivalents for read-only epoch snapshot access.

## 3. Storage & Persistence
- **L1 Cache:** In-Memory Partitioned Ring Buffers.
- **L2 Namespace Store:** **SQLite** (with WAL mode) or **RocksDB** for fast, local-first key-value retrieval.
- **Causal Ledger:** Append-only transaction log using **Protocol Buffers** for event serialization.

## 4. Diagnostics & Analysis
- **Tier 1 (Kernel):** Native Rust diagnostics.
- **Tier 2 (Heuristics):** Regex-based heuristic engine.
- **Tier 3 (Model):** Lightweight ML models (e.g., Llama-3-8B-Instruct via `llama.cpp` or remote gRPC calls) for asynchronous causal analysis.

## 5. Deployment & Observability
- **Infrastructure:** Sidecar deployment in **K8s (DaemonSets)** or **systemd** for localized agent processes.
- **Tracing:** **OpenTelemetry** integration for the "Causal Trace" dashboard.
- **Auth:** Mutual TLS (mTLS) for L2/L3 cross-node communication.

## 6. Coding Standards for Agents
- **Testing:** 100% unit test coverage for the Rust core; integration tests using Mock Agent simulations.
- **Safety:** Mandatory `unsafe` block documentation and audit for any pointers in the Shared Memory logic.
- **Performance:** No allocations in the L1 injection hot-path.
