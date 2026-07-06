// Rust port of PCG Random Number Generation.
//
// * Ported and modified by Voxell Paladynee in 2026.
// Derived from the original C implementation by Melissa O'Neill.
//
// Original C library copyright notice:
// Copyright 2014 Melissa O'Neill <oneill@pcg-random.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// For additional information about the PCG random number generation scheme,
// including its license and other licensing options, visit
//
//     http://www.pcg-random.org

//! ## `pcg_rs_raw` - raw translation of the equivalent C library  
//! 
//! The single file C library has been translated line by line into Rust with equivalent semantics,
//! additionally modularized into Rust modules that are documented. No additional changes were
//! made to the library.
#![allow(clippy::four_forward_slashes)]
#![allow(nonstandard_style)]

/// Output functions. These are the core of the PCG generation scheme.
///
/// visibility: private, do not use. use gen_* instead.
pub mod output;

/// Static initialization constants (if you can't call srandom for some bizarre reason).
///
/// visibility: public, avoid. use [`crate::seed`] instead for runtime seeding.
pub mod static_init;

/// Representations for the oneseq, mcg, and unique variants
///
/// visibility: public
pub mod reprs;

/// Multi-step advance functions (jump-ahead, jump-back)
///
/// visibility: private, do not use. use semi_private instead.
pub mod advance;

/// Functions to advance the underlying LCG, one version for each size and each style. These
/// functions are considered semi-private. There is rarely a good reason to call them directly.
///
/// visibility: public, avoid. use gen_* instead.
pub mod semi_private;

/// Functions to seed the RNG state, one version for each size and each style.  Unlike the step
/// functions, regular users can and should call these functions.
///
/// visibility: public
pub mod seed;

// Now, finally we create each of the individual generators. We provide
// a random_r function that provides a random number of the appropriate
// type (using the full range of the type) and a boundedrand_r version
// that provides
//
// Implementation notes for boundedrand_r:
//
//     To avoid bias, we need to make the range of the RNG a multiple of
//     bound, which we do by dropping output less than a threshold.
//     Let's consider a 32-bit case...  A naive scheme to calculate the
//     threshold would be to do
//
//         uint32_t threshold = 0x100000000ull % bound;
//
//     but 64-bit div/mod is slower than 32-bit div/mod (especially on
//     32-bit platforms).  In essence, we do
//
//         uint32_t threshold = (0x100000000ull-bound) % bound;
//
//     because this version will calculate the same modulus, but the LHS
//     value is less than 2^32.
//
//     (Note that using modulo is only wise for good RNGs, poorer RNGs
//     such as raw LCGs do better using a technique based on division.)
//     Empricical tests show that division is preferable to modulus for
//     reducting the range of an RNG.  It's faster, and sometimes it can
//     even be statistically prefereable.

/// no version with 8 bit state since these are reducing output function
///
/// visibility: public
pub mod gen_xsh_rs;

/// no version with 8 bit state since these are reducing output function
///
/// visibility: public
pub mod gen_xsh_rr;

/// no MCG versions because they don't make sense when you want to use the entire state
///
/// visibility: public
pub mod gen_rxs_m_xs;

/// only defined for "large" types
///
/// visibility: public
pub mod gen_xsl_rr;

/// only defined for "large" types
///
/// visibility: public
pub mod gen_xsl_rr_rr;

/// original typedefs from the C library.
///
/// visibility: public, avoid since they're not for Rust. use a crate wrapping this crate instead.
pub mod typedefs;
