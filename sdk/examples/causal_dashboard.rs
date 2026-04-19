use causalos_sdk::ipc::client::SidecarClient;
use causalos_sdk::kernel_proto::TraceRequest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = SidecarClient::new("causalos-kernel");
    client.connect_control_plane("http://[::1]:50051").await.map_err(|e| anyhow::anyhow!(e))?;
    
    println!("====================================================");
    println!("   CAUSALOS INSTITUTIONAL DASHBOARD (ALPHA)   ");
    println!("====================================================");
    println!("Connecting to Sidecar Kernel for Institutional Trace...");
    
    let trace_req = TraceRequest { plan_hash: "".to_string() };
    let trace = client.get_causal_trace(trace_req).await.map_err(|e| anyhow::anyhow!(e))?;
    
    println!("\n[CAUSAL CHAIN RECONSTRUCTION]");
    println!("----------------------------------------------------");
    for event in trace.events {
        let ts = event.timestamp;
        let event_type = event.event_type;
        let payload = event.payload;
        
        println!("[{}] {} >> {}", ts, event_type, payload);
    }
    println!("----------------------------------------------------");
    println!("End of Institutional Trace.");
    
    Ok(())
}
