use std::error::Error;
use tokio::net::windows::named_pipe::ServerOptions;
use tracing::info;

pub async fn run_named_pipe_server(pipe_name: &str) -> Result<(), Box<dyn Error>> {
    let pipe_path = format!(r"\\.\pipe\{}", pipe_name);
    info!("Starting Named Pipe server at {}", pipe_path);

    let server = ServerOptions::new()
        .first_pipe_instance(true)
        .create(&pipe_path)?;

    info!("Waiting for client connection...");
    
    // In a real implementation, we would loop and accept multiple connections.
    // For the alpha boilerplate, we'll demonstrate a single connection handler.
    loop {
        server.connect().await?;
        info!("Client connected to named pipe");
        
        // Handle client logic here (e.g., spawn a task)
        // For now, we'll just log and reset the connection for the next client.
        // Note: Real server would need to handle multiple instances correctly.
        
        // server.disconnect()?; // Disconnect to allow next connection
    }
}
