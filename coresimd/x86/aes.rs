//! AES New Instructions (AES-NI)
//!
//! The intrinsics here correspond to those in the `wmmintrin.h` C header.
//!
//! The reference is [Intel 64 and IA-32 Architectures Software Developer's
//! Manual Volume 2: Instruction Set Reference, A-Z][intel64_ref].
//!
//! [intel64_ref]: http://www.intel.de/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf

use coresimd::x86::__m128i;

#[cfg(test)]
use stdsimd_test::assert_instr;

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.x86.aesni.aesdec"]
    fn aesdec(a: __m128i, round_key: __m128i) -> __m128i;
    #[link_name = "llvm.x86.aesni.aesdeclast"]
    fn aesdeclast(a: __m128i, round_key: __m128i) -> __m128i;
    #[link_name = "llvm.x86.aesni.aesenc"]
    fn aesenc(a: __m128i, round_key: __m128i) -> __m128i;
    #[link_name = "llvm.x86.aesni.aesenclast"]
    fn aesenclast(a: __m128i, round_key: __m128i) -> __m128i;
    #[link_name = "llvm.x86.aesni.aesimc"]
    fn aesimc(a: __m128i) -> __m128i;
    #[link_name = "llvm.x86.aesni.aeskeygenassist"]
    fn aeskeygenassist(a: __m128i, imm8: u8) -> __m128i;
}

/// Perform one round of an AES decryption flow on data (state) in `a`.
#[inline]
#[target_feature(enable = "aes")]
#[cfg_attr(test, assert_instr(aesdec))]
pub unsafe fn _mm_aesdec_si128(a: __m128i, round_key: __m128i) -> __m128i {
    aesdec(a, round_key)
}

/// Perform the last round of an AES decryption flow on data (state) in `a`.
#[inline]
#[target_feature(enable = "aes")]
#[cfg_attr(test, assert_instr(aesdeclast))]
pub unsafe fn _mm_aesdeclast_si128(a: __m128i, round_key: __m128i) -> __m128i {
    aesdeclast(a, round_key)
}

/// Perform one round of an AES encryption flow on data (state) in `a`.
#[inline]
#[target_feature(enable = "aes")]
#[cfg_attr(test, assert_instr(aesenc))]
pub unsafe fn _mm_aesenc_si128(a: __m128i, round_key: __m128i) -> __m128i {
    aesenc(a, round_key)
}

/// Perform the last round of an AES encryption flow on data (state) in `a`.
#[inline]
#[target_feature(enable = "aes")]
#[cfg_attr(test, assert_instr(aesenclast))]
pub unsafe fn _mm_aesenclast_si128(a: __m128i, round_key: __m128i) -> __m128i {
    aesenclast(a, round_key)
}

/// Perform the `InvMixColumns` transformation on `a`.
#[inline]
#[target_feature(enable = "aes")]
#[cfg_attr(test, assert_instr(aesimc))]
pub unsafe fn _mm_aesimc_si128(a: __m128i) -> __m128i {
    aesimc(a)
}

/// Assist in expanding the AES cipher key.
///
/// Assist in expanding the AES cipher key by computing steps towards
/// generating a round key for encryption cipher using data from `a` and an
/// 8-bit round constant `imm8`.
#[inline]
#[target_feature(enable = "aes")]
#[cfg_attr(test, assert_instr(aeskeygenassist, imm8 = 0))]
#[rustc_args_required_const(1)]
pub unsafe fn _mm_aeskeygenassist_si128(a: __m128i, imm8: i32) -> __m128i {
    macro_rules! call {
        ($imm8: expr) => {
            aeskeygenassist(a, $imm8)
        };
    }
    constify_imm8!(imm8, call)
}

#[cfg(test)]
mod tests {
    // The constants in the tests below are just bit patterns. They should not
    // be interpreted as integers; signedness does not make sense for them, but
    // __m128i happens to be defined in terms of signed integers.
    #![allow(overflowing_literals)]

    use stdsimd_test::simd_test;

    use coresimd::x86::*;

    #[simd_test = "aes"]
    unsafe fn test_mm_aesdec_si128() {
        // Constants taken from https://msdn.microsoft.com/en-us/library/cc664949.aspx.
        let a = _mm_set_epi64x(0x0123456789abcdef, 0x8899aabbccddeeff);
        let k = _mm_set_epi64x(0x1133557799bbddff, 0x0022446688aaccee);
        let e = _mm_set_epi64x(0x044e4f5176fec48f, 0xb57ecfa381da39ee);
        let r = _mm_aesdec_si128(a, k);
        assert_eq_m128i(r, e);
    }

    #[simd_test = "aes"]
    unsafe fn test_mm_aesdeclast_si128() {
        // Constants taken from https://msdn.microsoft.com/en-us/library/cc714178.aspx.
        let a = _mm_set_epi64x(0x0123456789abcdef, 0x8899aabbccddeeff);
        let k = _mm_set_epi64x(0x1133557799bbddff, 0x0022446688aaccee);
        let e = _mm_set_epi64x(0x36cad57d9072bf9e, 0xf210dd981fa4a493);
        let r = _mm_aesdeclast_si128(a, k);
        assert_eq_m128i(r, e);
    }

    #[simd_test = "aes"]
    unsafe fn test_mm_aesenc_si128() {
        // Constants taken from https://msdn.microsoft.com/en-us/library/cc664810.aspx.
        let a = _mm_set_epi64x(0x0123456789abcdef, 0x8899aabbccddeeff);
        let k = _mm_set_epi64x(0x1133557799bbddff, 0x0022446688aaccee);
        let e = _mm_set_epi64x(0x16ab0e57dfc442ed, 0x28e4ee1884504333);
        let r = _mm_aesenc_si128(a, k);
        assert_eq_m128i(r, e);
    }

    #[simd_test = "aes"]
    unsafe fn test_mm_aesenclast_si128() {
        // Constants taken from https://msdn.microsoft.com/en-us/library/cc714136.aspx.
        let a = _mm_set_epi64x(0x0123456789abcdef, 0x8899aabbccddeeff);
        let k = _mm_set_epi64x(0x1133557799bbddff, 0x0022446688aaccee);
        let e = _mm_set_epi64x(0xb6dd7df25d7ab320, 0x4b04f98cf4c860f8);
        let r = _mm_aesenclast_si128(a, k);
        assert_eq_m128i(r, e);
    }

    #[simd_test = "aes"]
    unsafe fn test_mm_aesimc_si128() {
        // Constants taken from https://msdn.microsoft.com/en-us/library/cc714195.aspx.
        let a = _mm_set_epi64x(0x0123456789abcdef, 0x8899aabbccddeeff);
        let e = _mm_set_epi64x(0xc66c82284ee40aa0, 0x6633441122770055);
        let r = _mm_aesimc_si128(a);
        assert_eq_m128i(r, e);
    }

    #[simd_test = "aes"]
    unsafe fn test_mm_aeskeygenassist_si128() {
        // Constants taken from https://msdn.microsoft.com/en-us/library/cc714138.aspx.
        let a = _mm_set_epi64x(0x0123456789abcdef, 0x8899aabbccddeeff);
        let e = _mm_set_epi64x(0x857c266b7c266e85, 0xeac4eea9c4eeacea);
        let r = _mm_aeskeygenassist_si128(a, 5);
        assert_eq_m128i(r, e);
    }
}
