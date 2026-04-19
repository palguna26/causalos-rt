# Findings: CausalOS Agent Runtime

## Core Discovered Requirements
- **Goal**: CausalOS aims to bring institutional-grade reliability, overcoming current limits of agent architectures (unreliable memory, lack of firm safety gates, latency overloads).
- **Latency Target**: p99 < 100ms for L1 injection, and < 20ms for L1 cache injection.
- **Pillars**: Causal Memory (L1 cache, L2 team memory, L3 global), Governance Harness (Contracts, 2PC, Safe abort), Performance (Split-plane).
- **Metrics**: >90% reduction in repeated failures, 100% block of invariant-violating tools, <10% overhead on agent task time.
- **Architecture Limits**: Not building a new LLM, not a general human OS. Eventual consistency is accepted for global state (L2/L3).

## Phase 1 Discoveries
- Target architecture uses Rust for the sidecar event loop.
- Binary IPC over Unix Domain Sockets (UDS) are planned; Protobuf used for data serialization.

## Phase 2 Discoveries
- Embedded SDK uses Rust + WASM for cross-platform integration to the user agent side.
- Memory works via segment managers for "Read-only versioned Shared Memory".

## Phase 3 Discoveries
- Plan contracts enforce safety on tool calls with a Pre/Post (Two-Phase) hook structure.

## Phase 4 Discoveries
- Demands active observability using OpenTelemetry.
- Needs stripping of PII/Secrets for CNS propagation.

## Notes & Open Items
- Will need to determine baseline test suite methodology to confirm latency < 20ms.
- Will need mock scripts designed to stress UDS channels to ensure no packet dropping.
