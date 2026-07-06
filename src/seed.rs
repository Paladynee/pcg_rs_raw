//! Functions to seed the RNG state, one version for each size and each style.  Unlike the step
//! functions, regular users can and should call these functions.
//! 
//! visibility: public

use crate::{
    reprs::{
        pcg_state_8, pcg_state_16, pcg_state_32, pcg_state_64, pcg_state_128, pcg_state_setseq_8,
        pcg_state_setseq_16, pcg_state_setseq_32, pcg_state_setseq_64, pcg_state_setseq_128,
    },
    semi_private::{
        pcg_oneseq_8_step_r, pcg_oneseq_16_step_r, pcg_oneseq_32_step_r, pcg_oneseq_64_step_r,
        pcg_oneseq_128_step_r, pcg_setseq_8_step_r, pcg_setseq_16_step_r, pcg_setseq_32_step_r,
        pcg_setseq_64_step_r, pcg_setseq_128_step_r, pcg_unique_8_step_r, pcg_unique_16_step_r,
        pcg_unique_32_step_r, pcg_unique_64_step_r, pcg_unique_128_step_r,
    },
};

#[inline]
pub const fn pcg_oneseq_8_srandom_r(rng: &mut pcg_state_8, initstate: u8) {
    rng.state = 0;
    pcg_oneseq_8_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_oneseq_8_step_r(rng);
}

#[inline]
pub const fn pcg_mcg_8_srandom_r(rng: &mut pcg_state_8, initstate: u8) {
    rng.state = initstate | 1;
}

#[inline]
pub fn pcg_unique_8_srandom_r(rng: &mut pcg_state_8, initstate: u8) {
    rng.state = 0;
    pcg_unique_8_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_unique_8_step_r(rng);
}

#[inline]
pub const fn pcg_setseq_8_srandom_r(rng: &mut pcg_state_setseq_8, initstate: u8, initseq: u8) {
    rng.state = 0;
    rng.inc = (initseq << 1) | 1;
    pcg_setseq_8_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_setseq_8_step_r(rng);
}

#[inline]
pub const fn pcg_oneseq_16_srandom_r(rng: &mut pcg_state_16, initstate: u16) {
    rng.state = 0;
    pcg_oneseq_16_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_oneseq_16_step_r(rng);
}

#[inline]
pub const fn pcg_mcg_16_srandom_r(rng: &mut pcg_state_16, initstate: u16) {
    rng.state = initstate | 1;
}

#[inline]
pub fn pcg_unique_16_srandom_r(rng: &mut pcg_state_16, initstate: u16) {
    rng.state = 0;
    pcg_unique_16_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_unique_16_step_r(rng);
}

#[inline]
pub const fn pcg_setseq_16_srandom_r(rng: &mut pcg_state_setseq_16, initstate: u16, initseq: u16) {
    rng.state = 0;
    rng.inc = (initseq << 1) | 1;
    pcg_setseq_16_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_setseq_16_step_r(rng);
}

#[inline]
pub const fn pcg_oneseq_32_srandom_r(rng: &mut pcg_state_32, initstate: u32) {
    rng.state = 0;
    pcg_oneseq_32_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_oneseq_32_step_r(rng);
}

#[inline]
pub const fn pcg_mcg_32_srandom_r(rng: &mut pcg_state_32, initstate: u32) {
    rng.state = initstate | 1;
}

#[inline]
pub fn pcg_unique_32_srandom_r(rng: &mut pcg_state_32, initstate: u32) {
    rng.state = 0;
    pcg_unique_32_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_unique_32_step_r(rng);
}

#[inline]
pub const fn pcg_setseq_32_srandom_r(rng: &mut pcg_state_setseq_32, initstate: u32, initseq: u32) {
    rng.state = 0;
    rng.inc = (initseq << 1) | 1;
    pcg_setseq_32_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_setseq_32_step_r(rng);
}

#[inline]
pub const fn pcg_oneseq_64_srandom_r(rng: &mut pcg_state_64, initstate: u64) {
    rng.state = 0;
    pcg_oneseq_64_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_oneseq_64_step_r(rng);
}

#[inline]
pub const fn pcg_mcg_64_srandom_r(rng: &mut pcg_state_64, initstate: u64) {
    rng.state = initstate | 1;
}

#[inline]
pub fn pcg_unique_64_srandom_r(rng: &mut pcg_state_64, initstate: u64) {
    rng.state = 0;
    pcg_unique_64_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_unique_64_step_r(rng);
}

#[inline]
pub const fn pcg_setseq_64_srandom_r(rng: &mut pcg_state_setseq_64, initstate: u64, initseq: u64) {
    rng.state = 0;
    rng.inc = (initseq << 1) | 1;
    pcg_setseq_64_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_setseq_64_step_r(rng);
}

#[inline]
pub const fn pcg_oneseq_128_srandom_r(rng: &mut pcg_state_128, initstate: u128) {
    rng.state = 0;
    pcg_oneseq_128_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_oneseq_128_step_r(rng);
}

#[inline]
pub const fn pcg_mcg_128_srandom_r(rng: &mut pcg_state_128, initstate: u128) {
    rng.state = initstate | 1;
}

#[inline]
pub fn pcg_unique_128_srandom_r(rng: &mut pcg_state_128, initstate: u128) {
    rng.state = 0;
    pcg_unique_128_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_unique_128_step_r(rng);
}

#[inline]
pub const fn pcg_setseq_128_srandom_r(
    rng: &mut pcg_state_setseq_128,
    initstate: u128,
    initseq: u128,
) {
    rng.state = 0;
    rng.inc = (initseq << 1) | 1;
    pcg_setseq_128_step_r(rng);
    rng.state = rng.state.wrapping_add(initstate);
    pcg_setseq_128_step_r(rng);
}
