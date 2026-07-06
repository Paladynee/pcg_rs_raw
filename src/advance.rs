//! Multi-step advance functions (jump-ahead, jump-back)
//!
//! visibility: private, do not use.

#[inline]
pub const fn pcg_advance_lcg_8(state: u8, mut delta: u8, mut cur_mult: u8, mut cur_plus: u8) -> u8 {
    let mut acc_mult: u8 = 1;
    let mut acc_plus: u8 = 0;
    while delta > 0 {
        if delta & 1 != 0 {
            acc_mult = acc_mult.wrapping_mul(cur_mult);
            acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
        }
        cur_plus = (cur_mult.wrapping_add(1)).wrapping_mul(cur_plus);
        cur_mult = cur_mult.wrapping_mul(cur_mult);
        delta = delta >> 1;
    }
    acc_mult.wrapping_mul(state).wrapping_add(acc_plus)
}

#[inline]
pub const fn pcg_advance_lcg_16(
    state: u16,
    mut delta: u16,
    mut cur_mult: u16,
    mut cur_plus: u16,
) -> u16 {
    let mut acc_mult: u16 = 1;
    let mut acc_plus: u16 = 0;
    while delta > 0 {
        if delta & 1 != 0 {
            acc_mult = acc_mult.wrapping_mul(cur_mult);
            acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
        }
        cur_plus = (cur_mult.wrapping_add(1)).wrapping_mul(cur_plus);
        cur_mult = cur_mult.wrapping_mul(cur_mult);
        delta = delta >> 1;
    }
    acc_mult.wrapping_mul(state).wrapping_add(acc_plus)
}

#[inline]
pub const fn pcg_advance_lcg_32(
    state: u32,
    mut delta: u32,
    mut cur_mult: u32,
    mut cur_plus: u32,
) -> u32 {
    let mut acc_mult: u32 = 1;
    let mut acc_plus: u32 = 0;
    while delta > 0 {
        if delta & 1 != 0 {
            acc_mult = acc_mult.wrapping_mul(cur_mult);
            acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
        }
        cur_plus = (cur_mult.wrapping_add(1)).wrapping_mul(cur_plus);
        cur_mult = cur_mult.wrapping_mul(cur_mult);
        delta = delta >> 1;
    }
    acc_mult.wrapping_mul(state).wrapping_add(acc_plus)
}

#[inline]
pub const fn pcg_advance_lcg_64(
    state: u64,
    mut delta: u64,
    mut cur_mult: u64,
    mut cur_plus: u64,
) -> u64 {
    let mut acc_mult: u64 = 1;
    let mut acc_plus: u64 = 0;
    while delta > 0 {
        if delta & 1 != 0 {
            acc_mult = acc_mult.wrapping_mul(cur_mult);
            acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
        }
        cur_plus = (cur_mult.wrapping_add(1)).wrapping_mul(cur_plus);
        cur_mult = cur_mult.wrapping_mul(cur_mult);
        delta = delta >> 1;
    }
    acc_mult.wrapping_mul(state).wrapping_add(acc_plus)
}

#[inline]
pub const fn pcg_advance_lcg_128(
    state: u128,
    mut delta: u128,
    mut cur_mult: u128,
    mut cur_plus: u128,
) -> u128 {
    let mut acc_mult: u128 = 1;
    let mut acc_plus: u128 = 0;
    while delta > 0 {
        if delta & 1 != 0 {
            acc_mult = acc_mult.wrapping_mul(cur_mult);
            acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
        }
        cur_plus = (cur_mult.wrapping_add(1)).wrapping_mul(cur_plus);
        cur_mult = cur_mult.wrapping_mul(cur_mult);
        delta = delta >> 1;
    }
    acc_mult.wrapping_mul(state).wrapping_add(acc_plus)
}
