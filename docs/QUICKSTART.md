# QuickStart Guide: CausalOS

This guide will help you set up and run the CausalOS Agent Runtime for the first time.

## 1. Prerequisites

### Rust Toolchain
Install Rust (stable) from [rustup.rs](https://rustup.rs/).
```bash
rustc --version
```

### Protocol Buffers Compiler (protoc)
The kernel requires `protoc` to be installed and in your PATH. 
- **Windows**: Download from [protobuf releases](https://github.com/protocolbuffers/protobuf/releases), extract to `C:\protoc`, and add `C:\protoc\bin` to your System PATH.
- Verify:
  ```powershell
  $env:PROTOC = "C:\protoc\bin\protoc.exe" # Set explicitly if not in PATH
  protoc --version
  ```

## 2. Setting Up the Kernel

Clone the repository and build the Sidecar Kernel:
```powershell
cd causalos-runtime
$env:PROTOC = "C:\protoc\bin\protoc.exe"
cargo build -p sidecar
```

## 3. Running the Sidecar

The Sidecar is the "Kernel" of the system. It must be running for agents to work.
```powershell
cargo run -p sidecar
```
Wait until you see:
`INFO sidecar: Sidecar is ready. Institutional Memory enabled.`

## 4. Running the Learning Loop (Simulation)

Open a new terminal and run the integrated simulation. This will:
1.  Connect to the Sidecar over gRPC.
2.  Evaluate a plan.
3.  Record a successful outcome (triggering Promotion).
4.  Record a failure (triggering Negative Reinforcement).

```powershell
$env:PROTOC = "C:\protoc\bin\protoc.exe"
cargo run --example learning_loop
```

## 5. Visualizing the Trace (Dashboard)

After running the loop, check the **Institutional Trace** to see what the system learned:
```powershell
cargo run --example causal_dashboard
```

## 🎯 Next Steps
- Explore the **[API Specification](API_SPEC.md)** to integrate your own agent.
- Read about the **[Governance Engine](ARCHITECTURE.md#21-control-plane-the-sidecar-kernel)** to set custom safety invariants.
