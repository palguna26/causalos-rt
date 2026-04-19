# Agent Engineering Handbook: Building the CausalOS Runtime

This document serves as the mandatory guideline for any AI agent (or human engineer) contributing to the CausalOS Agent Runtime codebase.

## 1. Core Philosophy
- **Determinism Over Heuristics:** In the kernel, a decision must be reproducible. Avoid unseeded randomization.
- **Performance is a Security Feature:** A slow governance gate is a gate that will be bypassed. Target <20ms for hot-path L1 operations.
- **Fail Loudly & Safely:** If an invariant is violated or memory is corrupted, the system must halt and explain why, never silently ignore the error.

## 2. Engineering Rules (Non-Negotiable)

### 2.1 Language & Quality
- **Rust First:** All core logic belongs in Rust.
- **Zero-Allocations:** No heap allocations in the L1 injection hot-path. Use pre-allocated buffers.
- **Safety Audits:** Every `unsafe` block in Rust must be accompanied by a `// SAFETY:` comment justifying the invariant.

### 2.2 Documentation
- **Do Not Build Blindly:** Every feature must refer to a section in `DESIGN_SPECS.md` or `ARCHITECTURE.md`. If a requirement is ambiguous, **ask the user** before implementing.
- **Self-Documenting Code:** Uses descriptive types (e.g., `EpochId` instead of `u64`).

### 2.3 Testing Requirements
- **Unit Tests:** Every module must have comprehensive unit tests.
- **Property-Based Testing:** Use `proptest` for the IPC and Ranking logic to ensure edge cases are handled.
- **Simulation Harness:** Use the provided mock-agent harness to test governance gates against malicious behavior.

## 3. How to Approach a Task

1.  **Read the Specs:** Before writing a line of code, read the related `.md` files in this directory.
2.  **Verify the Plane:** Identify if your code lives in the **Control Plane** (Sidecar) or **Data Plane** (Embedded).
3.  **Check the IPC:** If you are adding a new signal, ensure it is added to `API_SPEC.md` first.
4.  **Implement & Trace:** Ensure your code emits the necessary OpenTelemetry traces for the **Causal Trace** dashboard.

## 4. Resilience Checklist
- [ ] Does this survive an agent process crash?
- [ ] Is the data in Shared Memory read-only for the agent?
- [ ] Does the append-only log handle partial writes/corruption?
- [ ] Is there an L1-only fallback mode if the Sidecar is unreachable?
