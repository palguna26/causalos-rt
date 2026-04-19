use zerocopy::{IntoBytes, FromBytes, Unaligned, Immutable, KnownLayout};
use zerocopy::byteorder::U64;
use zerocopy::byteorder::U32;

pub const MAX_PATTERNS_PER_SEGMENT: usize = 64;
pub const SLOT_CONTENT_SIZE: usize = 1024;

#[repr(C)]
#[derive(IntoBytes, FromBytes, Unaligned, Immutable, KnownLayout, Debug, Clone, Copy)]
pub struct CausalEpochHeader {
    pub epoch_id: U64<zerocopy::byteorder::NativeEndian>,
    pub step_count: U32<zerocopy::byteorder::NativeEndian>,
    pub risk_score: u8,
    pub _padding: [u8; 3],
}

#[repr(C)]
#[derive(IntoBytes, FromBytes, Unaligned, Immutable, KnownLayout, Debug, Clone, Copy)]
pub struct CausalPatternSlot {
    pub pattern_id: U64<zerocopy::byteorder::NativeEndian>,
    pub risk_score: u8,
    pub content: [u8; SLOT_CONTENT_SIZE],
}

#[repr(C)]
#[derive(IntoBytes, FromBytes, Unaligned, Immutable, KnownLayout, Debug, Clone, Copy)]
pub struct L1Partition {
    pub head: U32<zerocopy::byteorder::NativeEndian>,
    pub tail: U32<zerocopy::byteorder::NativeEndian>,
    pub slots: [CausalPatternSlot; MAX_PATTERNS_PER_SEGMENT],
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepClass {
    Planning = 0,
    Tool = 1,
    PostTool = 2,
    Error = 3,
}
