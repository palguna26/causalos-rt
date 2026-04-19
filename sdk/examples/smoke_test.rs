use causalos_sdk::{init_sdk, StepClass};
use zerocopy::Ref;

fn main() -> Result<(), String> {
    println!("Connecting to CausalOS L1 Cache...");
    let mut cache = init_sdk().map_err(|e| e.to_string())?;
    
    println!("Initializing Header...");
    if let Some(header_ref) = cache.header_mut() {
        // Ref::into_mut is an associated function
        let header = Ref::into_mut(header_ref);
        header.epoch_id.set(1);
        header.step_count.set(10);
        header.risk_score = 5;
        println!("Header initialized: Epoch {}", header.epoch_id.get());
    }

    println!("Accessing Planning Partition...");
    if let Some(planning_ref) = cache.get_partition_mut(StepClass::Planning) {
        let planning = Ref::into_mut(planning_ref);
        planning.head.set(1);
        planning.slots[0].pattern_id.set(12345);
        planning.slots[0].risk_score = 2;
        let content = b"Sample planning context pattern";
        let len = content.len().min(1024);
        planning.slots[0].content[..len].copy_from_slice(&content[..len]);
        println!("Planning slot 0 set.");
    }

    println!("Success! Phase 2 SDK smoke test complete.");
    Ok(())
}
