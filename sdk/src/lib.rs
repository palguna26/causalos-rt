pub mod memory;
pub mod cache;
pub mod ipc;
pub mod types;

pub use cache::L1Cache;
pub use ipc::SidecarClient;
pub use ipc::kernel_proto;
pub use ipc::kernel_proto::*;
pub use types::{CausalEpochHeader, StepClass};

pub fn init_sdk() -> Result<L1Cache, String> {
    L1Cache::connect("Local\\CausalOS_L1")
}
