use crate::{
    advance::{
        pcg_advance_lcg_8, pcg_advance_lcg_16, pcg_advance_lcg_32, pcg_advance_lcg_64,
        pcg_advance_lcg_128,
    },
    reprs::{
        pcg_state_8, pcg_state_16, pcg_state_32, pcg_state_64, pcg_state_128, pcg_state_setseq_8,
        pcg_state_setseq_16, pcg_state_setseq_32, pcg_state_setseq_64, pcg_state_setseq_128,
    },
};

pub const PCG_DEFAULT_MULTIPLIER_8: u8 = 0x8D;
pub const PCG_DEFAULT_MULTIPLIER_16: u16 = 0x321D;
pub const PCG_DEFAULT_MULTIPLIER_32: u32 = 0x2C9277B5;
pub const PCG_DEFAULT_MULTIPLIER_64: u64 = 0x5851F42D4C957F2D;
pub const PCG_DEFAULT_MULTIPLIER_128: u128 = 0x2360ED051FC65DA44385DF649FCCF645;

pub const PCG_DEFAULT_INCREMENT_8: u8 = 0x4D;
pub const PCG_DEFAULT_INCREMENT_16: u16 = 0xBB75;
pub const PCG_DEFAULT_INCREMENT_32: u32 = 0xAC564B05;
pub const PCG_DEFAULT_INCREMENT_64: u64 = 0x14057B7EF767814F;
pub const PCG_DEFAULT_INCREMENT_128: u128 = 0x5851F42D4C957F2D14057B7EF767814F;

/* Functions to advance the underlying LCG, one version for each size and
 * each style.  These functions are considered semi-private.  There is rarely
 * a good reason to call them directly.
 */

#[inline]
pub const fn pcg_oneseq_8_step_r(rng: &mut pcg_state_8) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_8)
        .wrapping_add(PCG_DEFAULT_INCREMENT_8);
}

#[inline]
pub const fn pcg_oneseq_8_advance_r(rng: &mut pcg_state_8, delta: u8) {
    rng.state = pcg_advance_lcg_8(
        rng.state,
        delta,
        PCG_DEFAULT_MULTIPLIER_8,
        PCG_DEFAULT_INCREMENT_8,
    );
}

#[inline]
pub const fn pcg_mcg_8_step_r(rng: &mut pcg_state_8) {
    rng.state = rng.state.wrapping_mul(PCG_DEFAULT_MULTIPLIER_8);
}

#[inline]
pub const fn pcg_mcg_8_advance_r(rng: &mut pcg_state_8, delta: u8) {
    rng.state = pcg_advance_lcg_8(rng.state, delta, PCG_DEFAULT_MULTIPLIER_8, 0);
}

#[inline]
pub fn pcg_unique_8_step_r(rng: &mut pcg_state_8) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_8)
        .wrapping_add((((rng as *mut pcg_state_8).addr() as isize) | 1) as u8);
}

#[inline]
pub fn pcg_unique_8_advance_r(rng: &mut pcg_state_8, delta: u8) {
    rng.state = pcg_advance_lcg_8(
        rng.state,
        delta,
        PCG_DEFAULT_MULTIPLIER_8,
        (((rng as *mut pcg_state_8).addr() as isize) | 1) as u8,
    );
}

#[inline]
pub const fn pcg_setseq_8_step_r(rng: &mut pcg_state_setseq_8) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_8)
        .wrapping_add(rng.inc);
}

#[inline]
pub const fn pcg_setseq_8_advance_r(rng: &mut pcg_state_setseq_8, delta: u8) {
    rng.state = pcg_advance_lcg_8(rng.state, delta, PCG_DEFAULT_MULTIPLIER_8, rng.inc);
}

#[inline]
pub const fn pcg_oneseq_16_step_r(rng: &mut pcg_state_16) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_16)
        .wrapping_add(PCG_DEFAULT_INCREMENT_16);
}

#[inline]
pub const fn pcg_oneseq_16_advance_r(rng: &mut pcg_state_16, delta: u16) {
    rng.state = pcg_advance_lcg_16(
        rng.state,
        delta,
        PCG_DEFAULT_MULTIPLIER_16,
        PCG_DEFAULT_INCREMENT_16,
    );
}

#[inline]
pub const fn pcg_mcg_16_step_r(rng: &mut pcg_state_16) {
    rng.state = rng.state.wrapping_mul(PCG_DEFAULT_MULTIPLIER_16);
}

#[inline]
pub const fn pcg_mcg_16_advance_r(rng: &mut pcg_state_16, delta: u16) {
    rng.state = pcg_advance_lcg_16(rng.state, delta, PCG_DEFAULT_MULTIPLIER_16, 0);
}

#[inline]
pub fn pcg_unique_16_step_r(rng: &mut pcg_state_16) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_16)
        .wrapping_add((((rng as *mut pcg_state_16).addr() as isize) | 1) as u16);
}

#[inline]
pub fn pcg_unique_16_advance_r(rng: &mut pcg_state_16, delta: u16) {
    rng.state = pcg_advance_lcg_16(
        rng.state,
        delta,
        PCG_DEFAULT_MULTIPLIER_16,
        (((rng as *mut pcg_state_16).addr() as isize) | 1) as u16,
    );
}

#[inline]
pub const fn pcg_setseq_16_step_r(rng: &mut pcg_state_setseq_16) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_16)
        .wrapping_add(rng.inc);
}

#[inline]
pub const fn pcg_setseq_16_advance_r(rng: &mut pcg_state_setseq_16, delta: u16) {
    rng.state = pcg_advance_lcg_16(rng.state, delta, PCG_DEFAULT_MULTIPLIER_16, rng.inc);
}

#[inline]
pub const fn pcg_oneseq_32_step_r(rng: &mut pcg_state_32) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_32)
        .wrapping_add(PCG_DEFAULT_INCREMENT_32);
}

#[inline]
pub const fn pcg_oneseq_32_advance_r(rng: &mut pcg_state_32, delta: u32) {
    rng.state = pcg_advance_lcg_32(
        rng.state,
        delta,
        PCG_DEFAULT_MULTIPLIER_32,
        PCG_DEFAULT_INCREMENT_32,
    );
}

#[inline]
pub const fn pcg_mcg_32_step_r(rng: &mut pcg_state_32) {
    rng.state = rng.state.wrapping_mul(PCG_DEFAULT_MULTIPLIER_32);
}

#[inline]
pub const fn pcg_mcg_32_advance_r(rng: &mut pcg_state_32, delta: u32) {
    rng.state = pcg_advance_lcg_32(rng.state, delta, PCG_DEFAULT_MULTIPLIER_32, 0);
}

#[inline]
pub fn pcg_unique_32_step_r(rng: &mut pcg_state_32) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_32)
        .wrapping_add((((rng as *mut pcg_state_32).addr() as isize) | 1) as u32);
}

#[inline]
pub fn pcg_unique_32_advance_r(rng: &mut pcg_state_32, delta: u32) {
    rng.state = pcg_advance_lcg_32(
        rng.state,
        delta,
        PCG_DEFAULT_MULTIPLIER_32,
        (((rng as *mut pcg_state_32).addr() as isize) | 1) as u32,
    );
}

#[inline]
pub const fn pcg_setseq_32_step_r(rng: &mut pcg_state_setseq_32) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_32)
        .wrapping_add(rng.inc);
}

#[inline]
pub const fn pcg_setseq_32_advance_r(rng: &mut pcg_state_setseq_32, delta: u32) {
    rng.state = pcg_advance_lcg_32(rng.state, delta, PCG_DEFAULT_MULTIPLIER_32, rng.inc);
}

#[inline]
pub const fn pcg_oneseq_64_step_r(rng: &mut pcg_state_64) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_64)
        .wrapping_add(PCG_DEFAULT_INCREMENT_64);
}

#[inline]
pub const fn pcg_oneseq_64_advance_r(rng: &mut pcg_state_64, delta: u64) {
    rng.state = pcg_advance_lcg_64(
        rng.state,
        delta,
        PCG_DEFAULT_MULTIPLIER_64,
        PCG_DEFAULT_INCREMENT_64,
    );
}

#[inline]
pub const fn pcg_mcg_64_step_r(rng: &mut pcg_state_64) {
    rng.state = rng.state.wrapping_mul(PCG_DEFAULT_MULTIPLIER_64);
}

#[inline]
pub const fn pcg_mcg_64_advance_r(rng: &mut pcg_state_64, delta: u64) {
    rng.state = pcg_advance_lcg_64(rng.state, delta, PCG_DEFAULT_MULTIPLIER_64, 0);
}

#[inline]
pub fn pcg_unique_64_step_r(rng: &mut pcg_state_64) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_64)
        .wrapping_add((((rng as *mut pcg_state_64).addr() as isize) | 1) as u64);
}

#[inline]
pub fn pcg_unique_64_advance_r(rng: &mut pcg_state_64, delta: u64) {
    rng.state = pcg_advance_lcg_64(
        rng.state,
        delta,
        PCG_DEFAULT_MULTIPLIER_64,
        (((rng as *mut pcg_state_64).addr() as isize) | 1) as u64,
    );
}

#[inline]
pub const fn pcg_setseq_64_step_r(rng: &mut pcg_state_setseq_64) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_64)
        .wrapping_add(rng.inc);
}

#[inline]
pub const fn pcg_setseq_64_advance_r(rng: &mut pcg_state_setseq_64, delta: u64) {
    rng.state = pcg_advance_lcg_64(rng.state, delta, PCG_DEFAULT_MULTIPLIER_64, rng.inc);
}

#[inline]
pub const fn pcg_oneseq_128_step_r(rng: &mut pcg_state_128) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_128)
        .wrapping_add(PCG_DEFAULT_INCREMENT_128);
}

#[inline]
pub const fn pcg_oneseq_128_advance_r(rng: &mut pcg_state_128, delta: u128) {
    rng.state = pcg_advance_lcg_128(
        rng.state,
        delta,
        PCG_DEFAULT_MULTIPLIER_128,
        PCG_DEFAULT_INCREMENT_128,
    );
}

#[inline]
pub const fn pcg_mcg_128_step_r(rng: &mut pcg_state_128) {
    rng.state = rng.state.wrapping_mul(PCG_DEFAULT_MULTIPLIER_128);
}

#[inline]
pub const fn pcg_mcg_128_advance_r(rng: &mut pcg_state_128, delta: u128) {
    rng.state = pcg_advance_lcg_128(rng.state, delta, PCG_DEFAULT_MULTIPLIER_128, 0);
}

#[inline]
pub fn pcg_unique_128_step_r(rng: &mut pcg_state_128) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_128)
        .wrapping_add((((rng as *mut pcg_state_128).addr() as isize) | 1) as u128);
}

#[inline]
pub fn pcg_unique_128_advance_r(rng: &mut pcg_state_128, delta: u128) {
    rng.state = pcg_advance_lcg_128(
        rng.state,
        delta,
        PCG_DEFAULT_MULTIPLIER_128,
        (((rng as *mut pcg_state_128).addr() as isize) | 1) as u128,
    );
}

#[inline]
pub const fn pcg_setseq_128_step_r(rng: &mut pcg_state_setseq_128) {
    rng.state = rng
        .state
        .wrapping_mul(PCG_DEFAULT_MULTIPLIER_128)
        .wrapping_add(rng.inc);
}

#[inline]
pub const fn pcg_setseq_128_advance_r(rng: &mut pcg_state_setseq_128, delta: u128) {
    rng.state = pcg_advance_lcg_128(rng.state, delta, PCG_DEFAULT_MULTIPLIER_128, rng.inc);
}
