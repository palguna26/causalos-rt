# CausalOS Agent Runtime

[![Status Alpha](https://img.shields.io/badge/status-alpha-orange.svg)](https://github.com/palguna26/causalos-runtime)
[![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**CausalOS** is a specialized, high-performance "Agent Kernel" designed to provide deterministic governance, institutional memory, and split-plane execution for autonomous AI agents.

## 🧠 Why CausalOS?
Modern agents often operate in an "unconstrained" loop where failures are forgotten and risks are high. CausalOS solves this by providing:
- **Safety Plane**: Intercept and evaluate tool calls before execution.
- **Institutional Memory**: A persistent "Causal Ledger" that stores successful patterns and reinforced failures.
- **Micro-latency IPC**: Ultra-fast shared memory (L1 Cache) for real-time context injection.

## 🏗️ Architecture
CausalOS uses a **Split-Plane Architecture** to isolate the safety-critical Kernel from the high-frequency Agent loop.

```mermaid
graph TD
    subgraph Agent_Process
        Agent[AI Agent / LLM Loop]
        SDK[CausalOS SDK]
    end

    subgraph Sidecar_Kernel
        CP[Control Plane - gRPC]
        HP[Hot Path - Named Pipes]
        L1[Shared Memory - L1 Cache]
        Ledger[Causal Ledger - Institutional Memory]
    end

    Agent <--> SDK
    SDK -- "Evaluation/Promotion" --> CP
    SDK -- "Heartbeats/Injections" --> HP
    HP -- "Fast Read" --> L1
    CP -- "Persistence" --> Ledger
```

## 📂 Project Structure
- **[sidecar/](sidecar/)**: The Rust implementation of the CausalOS Kernel.
- **[sdk/](sdk/)**: The developer library for integrating agents into the CausalOS fabric.
- **[proto/](proto/)**: gRPC and Protobuf definitions for the kernel protocol.
- **[docs/](docs/)**: Detailed technical specifications and guides.

## 🚀 Quick Start
Ready to run your first causal loop? 
See the **[QuickStart Guide](docs/QUICKSTART.md)**.

## 📚 Documentation
- [Architecture & Design](docs/ARCHITECTURE.md)
- [API & Protocol Specification](docs/API_SPEC.md)
- [Technology Stack](docs/TECH_STACK.md)
- [Developer Roadmap](docs/ROADMAP.md)

---
*Built with ❤️ by the CausalOS Team.*
