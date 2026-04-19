# Product Requirements Document: CausalOS

## 1. Problem Statement
AI agents are increasingly autonomous but lack "Institutional Memory." They repeat errors, ignore historical successes, and provide no deterministic safety plane. Existing "memory" solutions (vector DBs) are too slow for real-time tool governance and lack the rigors of a kernel-level persistence layer.

## 2. Solution Overview
CausalOS is an **Agent Runtime** that acts as the "OS Kernel" for LLM-based agents. It provides a split-plane management system for safety, memory, and observability.

## 3. Core Features (Alpha Verified)

### 3.1 Deterministic Governance
- **Requirement**: Evaluates agent plans before execution.
- **Implementation**: Heuristic risk scoring and invariant enforcement via the Control Plane.

### 3.2 Cold-Start Institutional Memory (L1 Cache)
- **Requirement**: Inject relevant context in <10ms to avoid agent hallucination.
- **Implementation**: Zero-copy Win32 Shared Memory mapping (Hot Path).

### 3.3 The Causal Ledger
- **Requirement**: Audit-grade persistence of all causal transitions.
- **Implementation**: Binary append-only event store with PII sanitization.

### 3.4 Dynamic Ranking
- **Requirement**: Prioritize successful patterns in future reasoning.
- **Implementation**: Real-time reinforcement learning based on execution outcomes.

## 4. User Experience
- **Developer Flow**: Wrap the agent loop in the `SidecarClient`. The client handles the handshake, heartbeats, and contract negotiation with zero overhead.
- **Audit Flow**: Use the Trace Engine to reconstruct a "Causal Trace" (Why did the agent do X? What was the historical success rate of this action?).

## 5. Success Metrics
- **Reliability**: Measured by the reduction in "Repeat Failure" density over time.
- **Performance**: Hot-path signals (heartbeats/L1 reads) must resolve in <5ms.
- **Security**: 100% interception of tool calls defined in the Policy.
