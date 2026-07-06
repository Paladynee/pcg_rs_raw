use crate::{
    output::{
        pcg_output_xsh_rs_16_8, pcg_output_xsh_rs_32_16, pcg_output_xsh_rs_64_32,
        pcg_output_xsh_rs_128_64,
    },
    reprs::{
        pcg_state_16, pcg_state_32, pcg_state_64, pcg_state_128, pcg_state_setseq_16,
        pcg_state_setseq_32, pcg_state_setseq_64, pcg_state_setseq_128,
    },
    semi_private::{
        pcg_mcg_16_step_r, pcg_mcg_32_step_r, pcg_mcg_64_step_r, pcg_mcg_128_step_r,
        pcg_oneseq_16_step_r, pcg_oneseq_32_step_r, pcg_oneseq_64_step_r, pcg_oneseq_128_step_r,
        pcg_setseq_16_step_r, pcg_setseq_32_step_r, pcg_setseq_64_step_r, pcg_setseq_128_step_r,
        pcg_unique_16_step_r, pcg_unique_32_step_r, pcg_unique_64_step_r, pcg_unique_128_step_r,
    },
};

#[inline]
pub const fn pcg_oneseq_16_xsh_rs_8_random_r(rng: &mut pcg_state_16) -> u8 {
    let oldstate: u16 = rng.state;
    pcg_oneseq_16_step_r(rng);
    pcg_output_xsh_rs_16_8(oldstate)
}

#[inline]
pub const fn pcg_oneseq_16_xsh_rs_8_boundedrand_r(rng: &mut pcg_state_16, bound: u8) -> u8 {
    let threshold: u8 = ((-(bound as i8)) as u8) % bound;
    loop {
        let r: u8 = pcg_oneseq_16_xsh_rs_8_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_oneseq_32_xsh_rs_16_random_r(rng: &mut pcg_state_32) -> u16 {
    let oldstate: u32 = rng.state;
    pcg_oneseq_32_step_r(rng);
    pcg_output_xsh_rs_32_16(oldstate)
}

#[inline]
pub const fn pcg_oneseq_32_xsh_rs_16_boundedrand_r(rng: &mut pcg_state_32, bound: u16) -> u16 {
    let threshold: u16 = ((-(bound as i16)) as u16) % bound;
    loop {
        let r: u16 = pcg_oneseq_32_xsh_rs_16_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_oneseq_64_xsh_rs_32_random_r(rng: &mut pcg_state_64) -> u32 {
    let oldstate: u64 = rng.state;
    pcg_oneseq_64_step_r(rng);
    pcg_output_xsh_rs_64_32(oldstate)
}

#[inline]
pub const fn pcg_oneseq_64_xsh_rs_32_boundedrand_r(rng: &mut pcg_state_64, bound: u32) -> u32 {
    let threshold: u32 = ((-(bound as i32)) as u32) % bound;
    loop {
        let r: u32 = pcg_oneseq_64_xsh_rs_32_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_oneseq_128_xsh_rs_64_random_r(rng: &mut pcg_state_128) -> u64 {
    let oldstate: u128 = rng.state;
    pcg_oneseq_128_step_r(rng);
    pcg_output_xsh_rs_128_64(oldstate)
}

#[inline]
pub const fn pcg_oneseq_128_xsh_rs_64_boundedrand_r(rng: &mut pcg_state_128, bound: u64) -> u64 {
    let threshold: u64 = ((-(bound as i64)) as u64) % bound;
    loop {
        let r: u64 = pcg_oneseq_128_xsh_rs_64_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub fn pcg_unique_16_xsh_rs_8_random_r(rng: &mut pcg_state_16) -> u8 {
    let oldstate: u16 = rng.state;
    pcg_unique_16_step_r(rng);
    pcg_output_xsh_rs_16_8(oldstate)
}

#[inline]
pub fn pcg_unique_16_xsh_rs_8_boundedrand_r(rng: &mut pcg_state_16, bound: u8) -> u8 {
    let threshold: u8 = ((-(bound as i8)) as u8) % bound;
    loop {
        let r: u8 = pcg_unique_16_xsh_rs_8_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub fn pcg_unique_32_xsh_rs_16_random_r(rng: &mut pcg_state_32) -> u16 {
    let oldstate: u32 = rng.state;
    pcg_unique_32_step_r(rng);
    pcg_output_xsh_rs_32_16(oldstate)
}

#[inline]
pub fn pcg_unique_32_xsh_rs_16_boundedrand_r(rng: &mut pcg_state_32, bound: u16) -> u16 {
    let threshold: u16 = ((-(bound as i16)) as u16) % bound;
    loop {
        let r: u16 = pcg_unique_32_xsh_rs_16_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub fn pcg_unique_64_xsh_rs_32_random_r(rng: &mut pcg_state_64) -> u32 {
    let oldstate: u64 = rng.state;
    pcg_unique_64_step_r(rng);
    pcg_output_xsh_rs_64_32(oldstate)
}

#[inline]
pub fn pcg_unique_64_xsh_rs_32_boundedrand_r(rng: &mut pcg_state_64, bound: u32) -> u32 {
    let threshold: u32 = ((-(bound as i32)) as u32) % bound;
    loop {
        let r: u32 = pcg_unique_64_xsh_rs_32_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub fn pcg_unique_128_xsh_rs_64_random_r(rng: &mut pcg_state_128) -> u64 {
    let oldstate: u128 = rng.state;
    pcg_unique_128_step_r(rng);
    pcg_output_xsh_rs_128_64(oldstate)
}

#[inline]
pub fn pcg_unique_128_xsh_rs_64_boundedrand_r(rng: &mut pcg_state_128, bound: u64) -> u64 {
    let threshold: u64 = ((-(bound as i64)) as u64) % bound;
    loop {
        let r: u64 = pcg_unique_128_xsh_rs_64_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_setseq_16_xsh_rs_8_random_r(rng: &mut pcg_state_setseq_16) -> u8 {
    let oldstate: u16 = rng.state;
    pcg_setseq_16_step_r(rng);
    pcg_output_xsh_rs_16_8(oldstate)
}

#[inline]
pub const fn pcg_setseq_16_xsh_rs_8_boundedrand_r(rng: &mut pcg_state_setseq_16, bound: u8) -> u8 {
    let threshold: u8 = ((-(bound as i8)) as u8) % bound;
    loop {
        let r: u8 = pcg_setseq_16_xsh_rs_8_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_setseq_32_xsh_rs_16_random_r(rng: &mut pcg_state_setseq_32) -> u16 {
    let oldstate: u32 = rng.state;
    pcg_setseq_32_step_r(rng);
    pcg_output_xsh_rs_32_16(oldstate)
}

#[inline]
pub const fn pcg_setseq_32_xsh_rs_16_boundedrand_r(
    rng: &mut pcg_state_setseq_32,
    bound: u16,
) -> u16 {
    let threshold: u16 = ((-(bound as i16)) as u16) % bound;
    loop {
        let r: u16 = pcg_setseq_32_xsh_rs_16_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_setseq_64_xsh_rs_32_random_r(rng: &mut pcg_state_setseq_64) -> u32 {
    let oldstate: u64 = rng.state;
    pcg_setseq_64_step_r(rng);
    pcg_output_xsh_rs_64_32(oldstate)
}

#[inline]
pub const fn pcg_setseq_64_xsh_rs_32_boundedrand_r(
    rng: &mut pcg_state_setseq_64,
    bound: u32,
) -> u32 {
    let threshold: u32 = ((-(bound as i32)) as u32) % bound;
    loop {
        let r: u32 = pcg_setseq_64_xsh_rs_32_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_setseq_128_xsh_rs_64_random_r(rng: &mut pcg_state_setseq_128) -> u64 {
    let oldstate: u128 = rng.state;
    pcg_setseq_128_step_r(rng);
    pcg_output_xsh_rs_128_64(oldstate)
}

#[inline]
pub const fn pcg_setseq_128_xsh_rs_64_boundedrand_r(
    rng: &mut pcg_state_setseq_128,
    bound: u64,
) -> u64 {
    let threshold: u64 = ((-(bound as i64)) as u64) % bound;
    loop {
        let r: u64 = pcg_setseq_128_xsh_rs_64_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_mcg_16_xsh_rs_8_random_r(rng: &mut pcg_state_16) -> u8 {
    let oldstate: u16 = rng.state;
    pcg_mcg_16_step_r(rng);
    pcg_output_xsh_rs_16_8(oldstate)
}

#[inline]
pub const fn pcg_mcg_16_xsh_rs_8_boundedrand_r(rng: &mut pcg_state_16, bound: u8) -> u8 {
    let threshold: u8 = ((-(bound as i8)) as u8) % bound;
    loop {
        let r: u8 = pcg_mcg_16_xsh_rs_8_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_mcg_32_xsh_rs_16_random_r(rng: &mut pcg_state_32) -> u16 {
    let oldstate: u32 = rng.state;
    pcg_mcg_32_step_r(rng);
    pcg_output_xsh_rs_32_16(oldstate)
}

#[inline]
pub const fn pcg_mcg_32_xsh_rs_16_boundedrand_r(rng: &mut pcg_state_32, bound: u16) -> u16 {
    let threshold: u16 = ((-(bound as i16)) as u16) % bound;
    loop {
        let r: u16 = pcg_mcg_32_xsh_rs_16_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_mcg_64_xsh_rs_32_random_r(rng: &mut pcg_state_64) -> u32 {
    let oldstate: u64 = rng.state;
    pcg_mcg_64_step_r(rng);
    pcg_output_xsh_rs_64_32(oldstate)
}

#[inline]
pub const fn pcg_mcg_64_xsh_rs_32_boundedrand_r(rng: &mut pcg_state_64, bound: u32) -> u32 {
    let threshold: u32 = ((-(bound as i32)) as u32) % bound;
    loop {
        let r: u32 = pcg_mcg_64_xsh_rs_32_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_mcg_128_xsh_rs_64_random_r(rng: &mut pcg_state_128) -> u64 {
    let oldstate: u128 = rng.state;
    pcg_mcg_128_step_r(rng);
    pcg_output_xsh_rs_128_64(oldstate)
}

#[inline]
pub const fn pcg_mcg_128_xsh_rs_64_boundedrand_r(rng: &mut pcg_state_128, bound: u64) -> u64 {
    let threshold: u64 = ((-(bound as i64)) as u64) % bound;
    loop {
        let r: u64 = pcg_mcg_128_xsh_rs_64_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}
