//! Output functions. These are the core of the PCG generation scheme.
//!
//! visibility: private, do not use. use gen_* instead.

// XSH RS

#[inline]
pub const fn pcg_output_xsh_rs_16_8(state: u16) -> u8 {
    (((state >> 7) ^ state) >> ((state >> 14).wrapping_add(3))) as u8
}

#[inline]
pub const fn pcg_output_xsh_rs_32_16(state: u32) -> u16 {
    (((state >> 11) ^ state) >> ((state >> 30).wrapping_add(11))) as u16
}

#[inline]
pub const fn pcg_output_xsh_rs_64_32(state: u64) -> u32 {
    (((state >> 22) ^ state) >> ((state >> 61).wrapping_add(22))) as u32
}

#[inline]
pub const fn pcg_output_xsh_rs_128_64(state: u128) -> u64 {
    (((state >> 43) ^ state) >> ((state >> 124).wrapping_add(45))) as u64
}

// XSH RR

#[inline]
pub const fn pcg_output_xsh_rr_16_8(state: u16) -> u8 {
    ((((state >> 5) ^ state) >> 5) as u8).rotate_right((state >> 13) as u32)
}

#[inline]
pub const fn pcg_output_xsh_rr_32_16(state: u32) -> u16 {
    ((((state >> 10) ^ state) >> 12) as u16).rotate_right(state >> 28)
}

#[inline]
pub const fn pcg_output_xsh_rr_64_32(state: u64) -> u32 {
    ((((state >> 18) ^ state) >> 27) as u32).rotate_right((state >> 59) as u32)
}

#[inline]
pub const fn pcg_output_xsh_rr_128_64(state: u128) -> u64 {
    ((((state >> 29) ^ state) >> 58) as u64).rotate_right((state >> 122) as u32)
}

// RXS M XS

#[inline]
pub const fn pcg_output_rxs_m_xs_8_8(state: u8) -> u8 {
    let word: u8 = ((state >> ((state >> 6).wrapping_add(2))) ^ state).wrapping_mul(0xD9);
    (word >> 6) ^ word
}

#[inline]
pub const fn pcg_output_rxs_m_xs_16_16(state: u16) -> u16 {
    let word: u16 = ((state >> ((state >> 13).wrapping_add(3))) ^ state).wrapping_mul(0xF2D9);
    (word >> 11) ^ word
}

#[inline]
pub const fn pcg_output_rxs_m_xs_32_32(state: u32) -> u32 {
    let word: u32 = ((state >> ((state >> 28).wrapping_add(4))) ^ state).wrapping_mul(0x108EF2D9);
    (word >> 22) ^ word
}

#[inline]
pub const fn pcg_output_rxs_m_xs_64_64(state: u64) -> u64 {
    let word: u64 =
        ((state >> ((state >> 59).wrapping_add(5))) ^ state).wrapping_mul(0xAEF17502108EF2D9);
    (word >> 43) ^ word
}

#[inline]
pub const fn pcg_output_rxs_m_xs_128_128(state: u128) -> u128 {
    let word: u128 = ((state >> ((state >> 122).wrapping_add(6))) ^ state)
        .wrapping_mul(0xF69019274D7F699CAEF17502108EF2D9);
    (word >> 86) ^ word
}

// XSL RR (only defined for >= 64 bits)

#[inline]
pub const fn pcg_output_xsl_rr_64_32(state: u64) -> u32 {
    (((state >> 32) as u32) ^ (state as u32)).rotate_right((state >> 59) as u32)
}

#[inline]
pub const fn pcg_output_xsl_rr_128_64(state: u128) -> u64 {
    (((state >> 64) as u64) ^ (state as u64)).rotate_right((state >> 122) as u32)
}

// XSL RR RR (only defined for >= 64 bits)

#[inline]
pub const fn pcg_output_xsl_rr_rr_64_64(state: u64) -> u64 {
    let high: u32 = (state >> 32) as u32;
    let newlow: u32 = (high ^ (state as u32)).rotate_right((state >> 59) as u32);
    let newhigh: u32 = high.rotate_right(newlow & 31);
    ((newhigh as u64) << 32) | newlow as u64
}

#[inline]
pub const fn pcg_output_xsl_rr_rr_128_128(state: u128) -> u128 {
    let high: u64 = (state >> 64) as u64;
    let newlow: u64 = (high ^ (state as u64)).rotate_right((state >> 122) as u32);
    let newhigh: u64 = high.rotate_right((newlow & 63) as u32);
    ((newhigh as u128) << 64) | newlow as u128
}
