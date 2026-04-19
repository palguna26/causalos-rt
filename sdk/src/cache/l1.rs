use crate::memory::MemorySegment;
use crate::types::{CausalEpochHeader, L1Partition, StepClass};
use zerocopy::Ref;
use std::mem::size_of;

pub const L1_SIZE: usize = 1024 * 1024; // 1MB segment

pub struct L1Cache {
    segment: MemorySegment,
}

impl L1Cache {
    pub fn connect(name: &str) -> Result<Self, String> {
        let segment = MemorySegment::create(name, L1_SIZE)?;
        Ok(Self { segment })
    }

    pub fn header(&self) -> Option<Ref<&[u8], CausalEpochHeader>> {
        Ref::new_unaligned(&self.segment.as_slice()[..size_of::<CausalEpochHeader>()])
    }

    pub fn header_mut(&mut self) -> Option<Ref<&mut [u8], CausalEpochHeader>> {
        Ref::new_unaligned(&mut self.segment.as_slice_mut()[..size_of::<CausalEpochHeader>()])
    }

    pub fn get_partition(&self, class: StepClass) -> Option<Ref<&[u8], L1Partition>> {
        let offset = self.get_partition_offset(class);
        Ref::new_unaligned(&self.segment.as_slice()[offset..offset + size_of::<L1Partition>()])
    }

    pub fn get_partition_mut(&mut self, class: StepClass) -> Option<Ref<&mut [u8], L1Partition>> {
        let offset = self.get_partition_offset(class);
        Ref::new_unaligned(&mut self.segment.as_slice_mut()[offset..offset + size_of::<L1Partition>()])
    }

    fn get_partition_offset(&self, class: StepClass) -> usize {
        let header_size = size_of::<CausalEpochHeader>();
        let partition_size = size_of::<L1Partition>();
        match class {
            StepClass::Planning => header_size,
            StepClass::Tool => header_size + partition_size,
            StepClass::Error | StepClass::PostTool => header_size + 2 * partition_size,
        }
    }
}
