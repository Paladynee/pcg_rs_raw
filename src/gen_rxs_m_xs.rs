use crate::{
    output::{
        pcg_output_rxs_m_xs_8_8, pcg_output_rxs_m_xs_16_16, pcg_output_rxs_m_xs_32_32,
        pcg_output_rxs_m_xs_64_64, pcg_output_rxs_m_xs_128_128,
    },
    reprs::{
        pcg_state_8, pcg_state_16, pcg_state_32, pcg_state_64, pcg_state_128, pcg_state_setseq_8,
        pcg_state_setseq_16, pcg_state_setseq_32, pcg_state_setseq_64, pcg_state_setseq_128,
    },
    semi_private::{
        pcg_oneseq_8_step_r, pcg_oneseq_16_step_r, pcg_oneseq_32_step_r, pcg_oneseq_64_step_r,
        pcg_oneseq_128_step_r, pcg_setseq_8_step_r, pcg_setseq_16_step_r, pcg_setseq_32_step_r,
        pcg_setseq_64_step_r, pcg_setseq_128_step_r, pcg_unique_16_step_r, pcg_unique_32_step_r,
        pcg_unique_64_step_r, pcg_unique_128_step_r,
    },
};

#[inline]
pub const fn pcg_oneseq_8_rxs_m_xs_8_random_r(rng: &mut pcg_state_8) -> u8 {
    let oldstate: u8 = rng.state;
    pcg_oneseq_8_step_r(rng);
    pcg_output_rxs_m_xs_8_8(oldstate)
}

#[inline]
pub const fn pcg_oneseq_8_rxs_m_xs_8_boundedrand_r(rng: &mut pcg_state_8, bound: u8) -> u8 {
    let threshold: u8 = ((-(bound as i8)) as u8) % bound;
    loop {
        let r: u8 = pcg_oneseq_8_rxs_m_xs_8_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_oneseq_16_rxs_m_xs_16_random_r(rng: &mut pcg_state_16) -> u16 {
    let oldstate: u16 = rng.state;
    pcg_oneseq_16_step_r(rng);
    pcg_output_rxs_m_xs_16_16(oldstate)
}

#[inline]
pub const fn pcg_oneseq_16_rxs_m_xs_16_boundedrand_r(rng: &mut pcg_state_16, bound: u16) -> u16 {
    let threshold: u16 = ((-(bound as i16)) as u16) % bound;
    loop {
        let r: u16 = pcg_oneseq_16_rxs_m_xs_16_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_oneseq_32_rxs_m_xs_32_random_r(rng: &mut pcg_state_32) -> u32 {
    let oldstate: u32 = rng.state;
    pcg_oneseq_32_step_r(rng);
    pcg_output_rxs_m_xs_32_32(oldstate)
}

#[inline]
pub const fn pcg_oneseq_32_rxs_m_xs_32_boundedrand_r(rng: &mut pcg_state_32, bound: u32) -> u32 {
    let threshold: u32 = ((-(bound as i32)) as u32) % bound;
    loop {
        let r: u32 = pcg_oneseq_32_rxs_m_xs_32_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_oneseq_64_rxs_m_xs_64_random_r(rng: &mut pcg_state_64) -> u64 {
    let oldstate: u64 = rng.state;
    pcg_oneseq_64_step_r(rng);
    pcg_output_rxs_m_xs_64_64(oldstate)
}

#[inline]
pub const fn pcg_oneseq_64_rxs_m_xs_64_boundedrand_r(rng: &mut pcg_state_64, bound: u64) -> u64 {
    let threshold: u64 = ((-(bound as i64)) as u64) % bound;
    loop {
        let r: u64 = pcg_oneseq_64_rxs_m_xs_64_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_oneseq_128_rxs_m_xs_128_random_r(rng: &mut pcg_state_128) -> u128 {
    let oldstate: u128 = rng.state;
    pcg_oneseq_128_step_r(rng);
    pcg_output_rxs_m_xs_128_128(oldstate)
}

#[inline]
pub const fn pcg_oneseq_128_rxs_m_xs_128_boundedrand_r(
    rng: &mut pcg_state_128,
    bound: u128,
) -> u128 {
    let threshold: u128 = ((-(bound as i128)) as u128) % bound;
    loop {
        let r: u128 = pcg_oneseq_128_rxs_m_xs_128_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

// no A_rxs_m_xs_8_random_r
// no pcg_unique_8_rxs_m_xs_8_boundedrand_r

#[inline]
pub fn pcg_unique_16_rxs_m_xs_16_random_r(rng: &mut pcg_state_16) -> u16 {
    let oldstate: u16 = rng.state;
    pcg_unique_16_step_r(rng);
    pcg_output_rxs_m_xs_16_16(oldstate)
}

#[inline]
pub fn pcg_unique_16_rxs_m_xs_16_boundedrand_r(rng: &mut pcg_state_16, bound: u16) -> u16 {
    let threshold: u16 = ((-(bound as i16)) as u16) % bound;
    loop {
        let r: u16 = pcg_unique_16_rxs_m_xs_16_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub fn pcg_unique_32_rxs_m_xs_32_random_r(rng: &mut pcg_state_32) -> u32 {
    let oldstate: u32 = rng.state;
    pcg_unique_32_step_r(rng);
    pcg_output_rxs_m_xs_32_32(oldstate)
}

#[inline]
pub fn pcg_unique_32_rxs_m_xs_32_boundedrand_r(rng: &mut pcg_state_32, bound: u32) -> u32 {
    let threshold: u32 = ((-(bound as i32)) as u32) % bound;
    loop {
        let r: u32 = pcg_unique_32_rxs_m_xs_32_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub fn pcg_unique_64_rxs_m_xs_64_random_r(rng: &mut pcg_state_64) -> u64 {
    let oldstate: u64 = rng.state;
    pcg_unique_64_step_r(rng);
    pcg_output_rxs_m_xs_64_64(oldstate)
}

#[inline]
pub fn pcg_unique_64_rxs_m_xs_64_boundedrand_r(rng: &mut pcg_state_64, bound: u64) -> u64 {
    let threshold: u64 = ((-(bound as i64)) as u64) % bound;
    loop {
        let r: u64 = pcg_unique_64_rxs_m_xs_64_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub fn pcg_unique_128_rxs_m_xs_128_random_r(rng: &mut pcg_state_128) -> u128 {
    let oldstate: u128 = rng.state;
    pcg_unique_128_step_r(rng);
    pcg_output_rxs_m_xs_128_128(oldstate)
}

#[inline]
pub fn pcg_unique_128_rxs_m_xs_128_boundedrand_r(rng: &mut pcg_state_128, bound: u128) -> u128 {
    let threshold: u128 = ((-(bound as i128)) as u128) % bound;
    loop {
        let r: u128 = pcg_unique_128_rxs_m_xs_128_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_setseq_8_rxs_m_xs_8_random_r(rng: &mut pcg_state_setseq_8) -> u8 {
    let oldstate: u8 = rng.state;
    pcg_setseq_8_step_r(rng);
    pcg_output_rxs_m_xs_8_8(oldstate)
}

#[inline]
pub const fn pcg_setseq_8_rxs_m_xs_8_boundedrand_r(rng: &mut pcg_state_setseq_8, bound: u8) -> u8 {
    let threshold: u8 = ((-(bound as i8)) as u8) % bound;
    loop {
        let r: u8 = pcg_setseq_8_rxs_m_xs_8_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_setseq_16_rxs_m_xs_16_random_r(rng: &mut pcg_state_setseq_16) -> u16 {
    let oldstate: u16 = rng.state;
    pcg_setseq_16_step_r(rng);
    pcg_output_rxs_m_xs_16_16(oldstate)
}

#[inline]
pub const fn pcg_setseq_16_rxs_m_xs_16_boundedrand_r(
    rng: &mut pcg_state_setseq_16,
    bound: u16,
) -> u16 {
    let threshold: u16 = ((-(bound as i16)) as u16) % bound;
    loop {
        let r: u16 = pcg_setseq_16_rxs_m_xs_16_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_setseq_32_rxs_m_xs_32_random_r(rng: &mut pcg_state_setseq_32) -> u32 {
    let oldstate: u32 = rng.state;
    pcg_setseq_32_step_r(rng);
    pcg_output_rxs_m_xs_32_32(oldstate)
}

#[inline]
pub const fn pcg_setseq_32_rxs_m_xs_32_boundedrand_r(
    rng: &mut pcg_state_setseq_32,
    bound: u32,
) -> u32 {
    let threshold: u32 = ((-(bound as i32)) as u32) % bound;
    loop {
        let r: u32 = pcg_setseq_32_rxs_m_xs_32_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_setseq_64_rxs_m_xs_64_random_r(rng: &mut pcg_state_setseq_64) -> u64 {
    let oldstate: u64 = rng.state;
    pcg_setseq_64_step_r(rng);
    pcg_output_rxs_m_xs_64_64(oldstate)
}

#[inline]
pub const fn pcg_setseq_64_rxs_m_xs_64_boundedrand_r(
    rng: &mut pcg_state_setseq_64,
    bound: u64,
) -> u64 {
    let threshold: u64 = ((-(bound as i64)) as u64) % bound;
    loop {
        let r: u64 = pcg_setseq_64_rxs_m_xs_64_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

#[inline]
pub const fn pcg_setseq_128_rxs_m_xs_128_random_r(rng: &mut pcg_state_setseq_128) -> u128 {
    let oldstate: u128 = rng.state;
    pcg_setseq_128_step_r(rng);
    pcg_output_rxs_m_xs_128_128(oldstate)
}

#[inline]
pub const fn pcg_setseq_128_rxs_m_xs_128_boundedrand_r(
    rng: &mut pcg_state_setseq_128,
    bound: u128,
) -> u128 {
    let threshold: u128 = ((-(bound as i128)) as u128) % bound;
    loop {
        let r: u128 = pcg_setseq_128_rxs_m_xs_128_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}
