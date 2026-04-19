use tokio::net::windows::named_pipe::ClientOptions;
use tokio::io::AsyncWriteExt;
use tonic::transport::Channel;

pub mod kernel_proto {
    tonic::include_proto!("causalos.kernel");
}

use kernel_proto::kernel_service_client::KernelServiceClient;
pub use kernel_proto::*;

pub struct SidecarClient {
    pipe_path: String,
    grpc_client: Option<KernelServiceClient<Channel>>,
}

impl SidecarClient {
    pub fn new(path: &str) -> Self {
        Self {
            pipe_path: path.to_string(),
            grpc_client: None,
        }
    }

    pub async fn connect_control_plane(&mut self, addr: &str) -> Result<(), tonic::transport::Error> {
        let client = KernelServiceClient::connect(addr.to_string()).await?;
        self.grpc_client = Some(client);
        Ok(())
    }

    pub async fn connect_hot_path(&self) -> Result<tokio::net::windows::named_pipe::NamedPipeClient, std::io::Error> {
        let client = ClientOptions::new().open(&self.pipe_path)?;
        Ok(client)
    }

    pub async fn evaluate_plan(&mut self, plan: PlanRequest) -> Result<PlanContract, String> {
        if let Some(client) = &mut self.grpc_client {
            let response = client.evaluate_plan(plan).await
                .map_err(|e| e.to_string())?;
            Ok(response.into_inner())
        } else {
            Err("Control plane not connected".to_string())
        }
    }

    pub async fn record_outcome(&mut self, req: OutcomeRequest) -> Result<OutcomeResponse, String> {
        if let Some(client) = &mut self.grpc_client {
            let response = client.record_outcome(req).await
                .map_err(|e| e.to_string())?;
            Ok(response.into_inner())
        } else {
            Err("Control plane not connected".to_string())
        }
    }

    pub async fn get_causal_trace(&mut self, req: TraceRequest) -> Result<TraceResponse, String> {
        if let Some(client) = &mut self.grpc_client {
            let response = client.get_causal_trace(req).await
                .map_err(|e| e.to_string())?;
            Ok(response.into_inner())
        } else {
            Err("Control plane not connected".to_string())
        }
    }

    pub async fn request_tool_permission(&mut self, tool: ToolCallRequest) -> Result<ToolCallVerdict, String> {
        if let Some(client) = &mut self.grpc_client {
            let response = client.prepare_tool_call(tool).await
                .map_err(|e| e.to_string())?;
            Ok(response.into_inner())
        } else {
            Err("Control plane not connected".to_string())
        }
    }

    pub async fn send_heartbeat(&self) -> Result<(), std::io::Error> {
        let mut client = self.connect_hot_path().await?;
        client.write_all(b"HEARTBEAT").await?;
        Ok(())
    }
}
