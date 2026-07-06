//! Representations for the oneseq, mcg, and unique variants
//!
//! visibility: public

pub struct pcg_state_8 {
    pub state: u8,
}

pub struct pcg_state_16 {
    pub state: u16,
}

pub struct pcg_state_32 {
    pub state: u32,
}

pub struct pcg_state_64 {
    pub state: u64,
}

pub struct pcg_state_128 {
    pub state: u128,
}

pub struct pcg_state_setseq_8 {
    pub state: u8,
    pub inc: u8,
}

pub struct pcg_state_setseq_16 {
    pub state: u16,
    pub inc: u16,
}

pub struct pcg_state_setseq_32 {
    pub state: u32,
    pub inc: u32,
}

pub struct pcg_state_setseq_64 {
    pub state: u64,
    pub inc: u64,
}

pub struct pcg_state_setseq_128 {
    pub state: u128,
    pub inc: u128,
}
