//! Static initialization constants (if you can't call srandom for some bizarre reason).
//!
//! visibility: public, avoid. use [`crate::seed`] instead for runtime seeding.
use crate::reprs::*;

pub const PCG_STATE_ONESEQ_8_INITIALIZER: pcg_state_8 = pcg_state_8 { state: 0xD7 };
pub const PCG_STATE_ONESEQ_16_INITIALIZER: pcg_state_16 = pcg_state_16 { state: 0x20DF };
pub const PCG_STATE_ONESEQ_32_INITIALIZER: pcg_state_32 = pcg_state_32 { state: 0x46B56677 };
pub const PCG_STATE_ONESEQ_64_INITIALIZER: pcg_state_64 = pcg_state_64 {
    state: 0x4D595DF4D0F33173,
};
pub const PCG_STATE_ONESEQ_128_INITIALIZER: pcg_state_128 = pcg_state_128 {
    state: 0xB8DC10E158A9239298046DF007EC0A53,
};

pub const PCG_STATE_UNIQUE_8_INITIALIZER: pcg_state_8 = PCG_STATE_ONESEQ_8_INITIALIZER;
pub const PCG_STATE_UNIQUE_16_INITIALIZER: pcg_state_16 = PCG_STATE_ONESEQ_16_INITIALIZER;
pub const PCG_STATE_UNIQUE_32_INITIALIZER: pcg_state_32 = PCG_STATE_ONESEQ_32_INITIALIZER;
pub const PCG_STATE_UNIQUE_64_INITIALIZER: pcg_state_64 = PCG_STATE_ONESEQ_64_INITIALIZER;
pub const PCG_STATE_UNIQUE_128_INITIALIZER: pcg_state_128 = PCG_STATE_ONESEQ_128_INITIALIZER;

pub const PCG_STATE_MCG_8_INITIALIZER: pcg_state_8 = pcg_state_8 { state: 0xE5 };
pub const PCG_STATE_MCG_16_INITIALIZER: pcg_state_16 = pcg_state_16 { state: 0xA5E5 };
pub const PCG_STATE_MCG_32_INITIALIZER: pcg_state_32 = pcg_state_32 { state: 0xD15EA5E5 };
pub const PCG_STATE_MCG_64_INITIALIZER: pcg_state_64 = pcg_state_64 {
    state: 0xCAFEF00DD15EA5E5,
};
pub const PCG_STATE_MCG_128_INITIALIZER: pcg_state_128 = pcg_state_128 {
    state: 0xCAFEF00DD15EA5E5,
};

pub const PCG_STATE_SETSEQ_8_INITIALIZER: pcg_state_setseq_8 = pcg_state_setseq_8 {
    state: 0x9B,
    inc: 0xDB,
};
pub const PCG_STATE_SETSEQ_16_INITIALIZER: pcg_state_setseq_16 = pcg_state_setseq_16 {
    state: 0xE39B,
    inc: 0x5BDB,
};
pub const PCG_STATE_SETSEQ_32_INITIALIZER: pcg_state_setseq_32 = pcg_state_setseq_32 {
    state: 0xEC02D89B,
    inc: 0x94B95BDB,
};
pub const PCG_STATE_SETSEQ_64_INITIALIZER: pcg_state_setseq_64 = pcg_state_setseq_64 {
    state: 0x853C49E6748FEA9B,
    inc: 0xDA3E39CB94B95BDB,
};
pub const PCG_STATE_SETSEQ_128_INITIALIZER: pcg_state_setseq_128 = pcg_state_setseq_128 {
    state: 0x979C9A98D84620057D3E9CB6CFE0549B,
    inc: 0x1DA3E39CB94B95BDB,
};
