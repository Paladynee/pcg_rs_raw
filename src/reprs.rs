//! Representations for the oneseq, mcg, and unique variants
//!
//! visibility: public

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct pcg_state_8 {
    pub state: u8,
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct pcg_state_16 {
    pub state: u16,
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct pcg_state_32 {
    pub state: u32,
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct pcg_state_64 {
    pub state: u64,
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct pcg_state_128 {
    pub state: u128,
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct pcg_state_setseq_8 {
    pub state: u8,
    pub inc: u8,
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct pcg_state_setseq_16 {
    pub state: u16,
    pub inc: u16,
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct pcg_state_setseq_32 {
    pub state: u32,
    pub inc: u32,
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct pcg_state_setseq_64 {
    pub state: u64,
    pub inc: u64,
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct pcg_state_setseq_128 {
    pub state: u128,
    pub inc: u128,
}
