use crate::{
    output::{pcg_output_xsl_rr_64_32, pcg_output_xsl_rr_128_64},
    reprs::{pcg_state_64, pcg_state_128, pcg_state_setseq_64, pcg_state_setseq_128},
    semi_private::{
        pcg_mcg_64_step_r, pcg_mcg_128_step_r, pcg_oneseq_64_step_r, pcg_oneseq_128_step_r,
        pcg_setseq_64_step_r, pcg_setseq_128_step_r, pcg_unique_64_step_r, pcg_unique_128_step_r,
    },
};

#[inline]
pub const fn pcg_oneseq_64_xsl_rr_32_random_r(rng: &mut pcg_state_64) -> u32 {
    let oldstate: u64 = rng.state;
    pcg_oneseq_64_step_r(rng);
    pcg_output_xsl_rr_64_32(oldstate)
}

#[inline]
pub const fn pcg_oneseq_64_xsl_rr_32_boundedrand_r(rng: &mut pcg_state_64, bound: u32) -> u32 {
    let threshold: u32 = ((-(bound as i32)) as u32) % bound;
    loop {
        let r: u32 = pcg_oneseq_64_xsl_rr_32_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_oneseq_128_xsl_rr_64_random_r(rng: &mut pcg_state_128) -> u64 {
    let oldstate: u128 = rng.state;
    pcg_oneseq_128_step_r(rng);
    pcg_output_xsl_rr_128_64(oldstate)
}

#[inline]
pub const fn pcg_oneseq_128_xsl_rr_64_boundedrand_r(rng: &mut pcg_state_128, bound: u64) -> u64 {
    let threshold: u64 = ((-(bound as i64)) as u64) % bound;
    loop {
        let r: u64 = pcg_oneseq_128_xsl_rr_64_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub fn pcg_unique_64_xsl_rr_32_random_r(rng: &mut pcg_state_64) -> u32 {
    let oldstate: u64 = rng.state;
    pcg_unique_64_step_r(rng);
    pcg_output_xsl_rr_64_32(oldstate)
}

#[inline]
pub fn pcg_unique_64_xsl_rr_32_boundedrand_r(rng: &mut pcg_state_64, bound: u32) -> u32 {
    let threshold: u32 = ((-(bound as i32)) as u32) % bound;
    loop {
        let r: u32 = pcg_unique_64_xsl_rr_32_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub fn pcg_unique_128_xsl_rr_64_random_r(rng: &mut pcg_state_128) -> u64 {
    let oldstate: u128 = rng.state;
    pcg_unique_128_step_r(rng);
    pcg_output_xsl_rr_128_64(oldstate)
}

#[inline]
pub fn pcg_unique_128_xsl_rr_64_boundedrand_r(rng: &mut pcg_state_128, bound: u64) -> u64 {
    let threshold: u64 = ((-(bound as i64)) as u64) % bound;
    loop {
        let r: u64 = pcg_unique_128_xsl_rr_64_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_setseq_64_xsl_rr_32_random_r(rng: &mut pcg_state_setseq_64) -> u32 {
    let oldstate: u64 = rng.state;
    pcg_setseq_64_step_r(rng);
    pcg_output_xsl_rr_64_32(oldstate)
}

#[inline]
pub const fn pcg_setseq_64_xsl_rr_32_boundedrand_r(
    rng: &mut pcg_state_setseq_64,
    bound: u32,
) -> u32 {
    let threshold: u32 = ((-(bound as i32)) as u32) % bound;
    loop {
        let r: u32 = pcg_setseq_64_xsl_rr_32_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_setseq_128_xsl_rr_64_random_r(rng: &mut pcg_state_setseq_128) -> u64 {
    let oldstate: u128 = rng.state;
    pcg_setseq_128_step_r(rng);
    pcg_output_xsl_rr_128_64(oldstate)
}

#[inline]
pub const fn pcg_setseq_128_xsl_rr_64_boundedrand_r(
    rng: &mut pcg_state_setseq_128,
    bound: u64,
) -> u64 {
    let threshold: u64 = ((-(bound as i64)) as u64) % bound;
    loop {
        let r: u64 = pcg_setseq_128_xsl_rr_64_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_mcg_64_xsl_rr_32_random_r(rng: &mut pcg_state_64) -> u32 {
    let oldstate: u64 = rng.state;
    pcg_mcg_64_step_r(rng);
    pcg_output_xsl_rr_64_32(oldstate)
}

#[inline]
pub const fn pcg_mcg_64_xsl_rr_32_boundedrand_r(rng: &mut pcg_state_64, bound: u32) -> u32 {
    let threshold: u32 = ((-(bound as i32)) as u32) % bound;
    loop {
        let r: u32 = pcg_mcg_64_xsl_rr_32_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_mcg_128_xsl_rr_64_random_r(rng: &mut pcg_state_128) -> u64 {
    let oldstate: u128 = rng.state;
    pcg_mcg_128_step_r(rng);
    pcg_output_xsl_rr_128_64(oldstate)
}

#[inline]
pub const fn pcg_mcg_128_xsl_rr_64_boundedrand_r(rng: &mut pcg_state_128, bound: u64) -> u64 {
    let threshold: u64 = ((-(bound as i64)) as u64) % bound;
    loop {
        let r: u64 = pcg_mcg_128_xsl_rr_64_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}
