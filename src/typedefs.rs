#![allow(static_mut_refs)]

macro_rules! definer {
    ($(
        #define $a:ident $b:path
    )* $(,)?) => {
        $(
            #[doc(hidden)]
            #[macro_export]
            macro_rules! $a {
                () => { $b }
            }
            pub use $a;
        )*
    };
}

use crate::reprs;

//// Typedefs
pub type pcg32_random_t = crate::reprs::pcg_state_setseq_64;
pub type pcg32s_random_t = crate::reprs::pcg_state_64;
pub type pcg32u_random_t = crate::reprs::pcg_state_64;
pub type pcg32f_random_t = crate::reprs::pcg_state_64;

definer! {
    //// random_r
    #define pcg32_random_r                  gen_xsh_rr::pcg_setseq_64_xsh_rr_32_random_r
    #define pcg32s_random_r                 gen_xsh_rr::pcg_oneseq_64_xsh_rr_32_random_r
    #define pcg32u_random_r                 gen_xsh_rr::pcg_unique_64_xsh_rr_32_random_r
    #define pcg32f_random_r                 gen_xsh_rs::pcg_mcg_64_xsh_rs_32_random_r
    //// boundedrand_r
    #define pcg32_boundedrand_r             gen_xsh_rr::pcg_setseq_64_xsh_rr_32_boundedrand_r
    #define pcg32s_boundedrand_r            gen_xsh_rr::pcg_oneseq_64_xsh_rr_32_boundedrand_r
    #define pcg32u_boundedrand_r            gen_xsh_rr::pcg_unique_64_xsh_rr_32_boundedrand_r
    #define pcg32f_boundedrand_r            gen_xsh_rs::pcg_mcg_64_xsh_rs_32_boundedrand_r
    //// srandom_r
    #define pcg32_srandom_r                 seed::pcg_setseq_64_srandom_r
    #define pcg32s_srandom_r                seed::pcg_oneseq_64_srandom_r
    #define pcg32u_srandom_r                seed::pcg_unique_64_srandom_r
    #define pcg32f_srandom_r                seed::pcg_mcg_64_srandom_r
    //// advance_r
    #define pcg32_advance_r                 semi_private::pcg_setseq_64_advance_r
    #define pcg32s_advance_r                semi_private::pcg_oneseq_64_advance_r
    #define pcg32u_advance_r                semi_private::pcg_unique_64_advance_r
    #define pcg32f_advance_r                semi_private::pcg_mcg_64_advance_r
}

//// Typedefs
pub type pcg64_random_t = reprs::pcg_state_setseq_128;
pub type pcg64s_random_t = reprs::pcg_state_128;
pub type pcg64u_random_t = reprs::pcg_state_128;
pub type pcg64f_random_t = reprs::pcg_state_128;

definer! {
    //// random_r
    #define pcg64_random_r                  gen_xsl_rr::pcg_setseq_128_xsl_rr_64_random_r
    #define pcg64s_random_r                 gen_xsl_rr::pcg_oneseq_128_xsl_rr_64_random_r
    #define pcg64u_random_r                 gen_xsl_rr::pcg_unique_128_xsl_rr_64_random_r
    #define pcg64f_random_r                 gen_xsl_rr::pcg_mcg_128_xsl_rr_64_random_r
    //// boundedrand_r
    #define pcg64_boundedrand_r             gen_xsl_rr::pcg_setseq_128_xsl_rr_64_boundedrand_r
    #define pcg64s_boundedrand_r            gen_xsl_rr::pcg_oneseq_128_xsl_rr_64_boundedrand_r
    #define pcg64u_boundedrand_r            gen_xsl_rr::pcg_unique_128_xsl_rr_64_boundedrand_r
    #define pcg64f_boundedrand_r            gen_xsl_rr::pcg_mcg_128_xsl_rr_64_boundedrand_r
    //// srandom_r
    #define pcg64_srandom_r                 seed::pcg_setseq_128_srandom_r
    #define pcg64s_srandom_r                seed::pcg_oneseq_128_srandom_r
    #define pcg64u_srandom_r                seed::pcg_unique_128_srandom_r
    #define pcg64f_srandom_r                seed::pcg_mcg_128_srandom_r
    //// advance_r
    #define pcg64_advance_r                 semi_private::pcg_setseq_128_advance_r
    #define pcg64s_advance_r                semi_private::pcg_oneseq_128_advance_r
    #define pcg64u_advance_r                semi_private::pcg_unique_128_advance_r
    #define pcg64f_advance_r                semi_private::pcg_mcg_128_advance_r
}

//// Typedefs
pub type pcg8si_random_t = crate::reprs::pcg_state_8;
pub type pcg16si_random_t = crate::reprs::pcg_state_16;
pub type pcg32si_random_t = crate::reprs::pcg_state_32;
pub type pcg64si_random_t = crate::reprs::pcg_state_64;
pub type pcg128si_random_t = crate::reprs::pcg_state_128;

definer! {
    //// random_r
    #define pcg8si_random_r                 gen_rxs_m_xs::pcg_oneseq_8_rxs_m_xs_8_random_r
    #define pcg16si_random_r                gen_rxs_m_xs::pcg_oneseq_16_rxs_m_xs_16_random_r
    #define pcg32si_random_r                gen_rxs_m_xs::pcg_oneseq_32_rxs_m_xs_32_random_r
    #define pcg64si_random_r                gen_rxs_m_xs::pcg_oneseq_64_rxs_m_xs_64_random_r
    #define pcg128si_random_r               gen_rxs_m_xs::pcg_oneseq_128_rxs_m_xs_128_random_r

    //// boundedrand_r
    #define pcg8si_boundedrand_r            gen_rxs_m_xs::pcg_oneseq_8_rxs_m_xs_8_boundedrand_r
    #define pcg16si_boundedrand_r           gen_rxs_m_xs::pcg_oneseq_16_rxs_m_xs_16_boundedrand_r
    #define pcg32si_boundedrand_r           gen_rxs_m_xs::pcg_oneseq_32_rxs_m_xs_32_boundedrand_r
    #define pcg64si_boundedrand_r           gen_rxs_m_xs::pcg_oneseq_64_rxs_m_xs_64_boundedrand_r
    #define pcg128si_boundedrand_r          gen_rxs_m_xs::pcg_oneseq_128_rxs_m_xs_128_boundedrand_r
    //// srandom_r
    #define pcg8si_srandom_r                seed::pcg_oneseq_8_srandom_r
    #define pcg16si_srandom_r               seed::pcg_oneseq_16_srandom_r
    #define pcg32si_srandom_r               seed::pcg_oneseq_32_srandom_r
    #define pcg64si_srandom_r               seed::pcg_oneseq_64_srandom_r
    #define pcg128si_srandom_r              seed::pcg_oneseq_128_srandom_r
    //// advance_r
    #define pcg8si_advance_r                semi_private::pcg_oneseq_8_advance_r
    #define pcg16si_advance_r               semi_private::pcg_oneseq_16_advance_r
    #define pcg32si_advance_r               semi_private::pcg_oneseq_32_advance_r
    #define pcg64si_advance_r               semi_private::pcg_oneseq_64_advance_r
    #define pcg128si_advance_r              semi_private::pcg_oneseq_128_advance_r
}

//// Typedefs
pub type pcg8i_random_t = reprs::pcg_state_setseq_8;
pub type pcg16i_random_t = reprs::pcg_state_setseq_16;
pub type pcg32i_random_t = reprs::pcg_state_setseq_32;
pub type pcg64i_random_t = reprs::pcg_state_setseq_64;
pub type pcg128i_random_t = reprs::pcg_state_setseq_128;

definer! {
    //// random_r
    #define pcg8i_random_r                  gen_rxs_m_xs::pcg_setseq_8_rxs_m_xs_8_random_r
    #define pcg16i_random_r                 gen_rxs_m_xs::pcg_setseq_16_rxs_m_xs_16_random_r
    #define pcg32i_random_r                 gen_rxs_m_xs::pcg_setseq_32_rxs_m_xs_32_random_r
    #define pcg64i_random_r                 gen_rxs_m_xs::pcg_setseq_64_rxs_m_xs_64_random_r
    #define pcg128i_random_r                gen_rxs_m_xs::pcg_setseq_128_rxs_m_xs_128_random_r
    //// boundedrand_r
    #define pcg8i_boundedrand_r             gen_rxs_m_xs::pcg_setseq_8_rxs_m_xs_8_boundedrand_r
    #define pcg16i_boundedrand_r            gen_rxs_m_xs::pcg_setseq_16_rxs_m_xs_16_boundedrand_r
    #define pcg32i_boundedrand_r            gen_rxs_m_xs::pcg_setseq_32_rxs_m_xs_32_boundedrand_r
    #define pcg64i_boundedrand_r            gen_rxs_m_xs::pcg_setseq_64_rxs_m_xs_64_boundedrand_r
    #define pcg128i_boundedrand_r           gen_rxs_m_xs::pcg_setseq_128_rxs_m_xs_128_boundedrand_r
    //// srandom_rv
    #define pcg8i_srandom_r                 seed::pcg_setseq_8_srandom_r
    #define pcg16i_srandom_r                seed::pcg_setseq_16_srandom_r
    #define pcg32i_srandom_r                seed::pcg_setseq_32_srandom_r
    #define pcg64i_srandom_r                seed::pcg_setseq_64_srandom_r
    #define pcg128i_srandom_r               seed::pcg_setseq_128_srandom_r
    //// advance_r
    #define pcg8i_advance_r                 semi_private::pcg_setseq_8_advance_r
    #define pcg16i_advance_r                semi_private::pcg_setseq_16_advance_r
    #define pcg32i_advance_r                semi_private::pcg_setseq_32_advance_r
    #define pcg64i_advance_r                semi_private::pcg_setseq_64_advance_r
    #define pcg128i_advance_r               semi_private::pcg_setseq_128_advance_r
}

/*
 * Static initialization constants (if you can't call srandom for some
 * bizarre reason).
 */

definer! {
    #define PCG32_INITIALIZER       static_init::PCG_STATE_SETSEQ_64_INITIALIZER
    #define PCG32U_INITIALIZER      static_init::PCG_STATE_UNIQUE_64_INITIALIZER
    #define PCG32S_INITIALIZER      static_init::PCG_STATE_ONESEQ_64_INITIALIZER
    #define PCG32F_INITIALIZER      static_init::PCG_STATE_MCG_64_INITIALIZER

    #define PCG64_INITIALIZER       static_init::PCG_STATE_SETSEQ_128_INITIALIZER
    #define PCG64U_INITIALIZER      static_init::PCG_STATE_UNIQUE_128_INITIALIZER
    #define PCG64S_INITIALIZER      static_init::PCG_STATE_ONESEQ_128_INITIALIZER
    #define PCG64F_INITIALIZER      static_init::PCG_STATE_MCG_128_INITIALIZER

    #define PCG8SI_INITIALIZER      static_init::PCG_STATE_ONESEQ_8_INITIALIZER
    #define PCG16SI_INITIALIZER     static_init::PCG_STATE_ONESEQ_16_INITIALIZER
    #define PCG32SI_INITIALIZER     static_init::PCG_STATE_ONESEQ_32_INITIALIZER
    #define PCG64SI_INITIALIZER     static_init::PCG_STATE_ONESEQ_64_INITIALIZER
    #define PCG128SI_INITIALIZER    static_init::PCG_STATE_ONESEQ_128_INITIALIZER

    #define PCG8I_INITIALIZER       static_init::PCG_STATE_SETSEQ_8_INITIALIZER
    #define PCG16I_INITIALIZER      static_init::PCG_STATE_SETSEQ_16_INITIALIZER
    #define PCG32I_INITIALIZER      static_init::PCG_STATE_SETSEQ_32_INITIALIZER
    #define PCG64I_INITIALIZER      static_init::PCG_STATE_SETSEQ_64_INITIALIZER
    #define PCG128I_INITIALIZER     static_init::PCG_STATE_SETSEQ_128_INITIALIZER
}
