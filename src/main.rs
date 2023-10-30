#[inline]
const fn mul128_u64(lowbits: u128, d: u64) -> u64 {
    let mut bottom_half = (lowbits & 0xFFFFFFFFFFFFFFFF) * d as u128;
    bottom_half >>= 64;
    let top_half = (lowbits >> 64) * d as u128;
    let both_halves = bottom_half + top_half;
    (both_halves >> 64) as u64
}

#[inline]
const fn compute_m_u64(d: u64) -> u128 {
    (0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF / d as u128) + 1
}
#[inline]
const fn fastmod_u64(a: u64, m: u128, d: u64) -> u64 {
    let lowbits = m.wrapping_mul(a as u128);
    mul128_u64(lowbits, d)
}
// for d > 1
#[inline]
const fn fastdiv_u64(a: u64, m: u128) -> u64 {
    mul128_u64(m, a)
}
#[inline]
const fn is_divisible_u64(n: u64, m: u128) -> bool {
    (n as u128).wrapping_mul(m) <= m - 1
}

/// Allows precomputing the division factor for fast division, modulo, and divisibility checks.
pub trait FastDiv: Copy {
    type PrecomputedDiv: Copy;
    /// Precompute the division factor from the divisor `self`.
    fn precompute_div(self) -> Self::PrecomputedDiv;
    /// Divide by the divisor, given the precomputed division factor.
    fn fast_div(self, precomputed: Self::PrecomputedDiv) -> Self;
    /// Compute the remainder of the division of `self` by the divisor, given the precomputed division factor and the divisor `d`.
    /// If the precomputed division factor does not come from the same provided divisor, the
    /// result is unspecified.
    fn fast_mod(self, precomputed: Self::PrecomputedDiv, d: Self) -> Self;
    /// Check if `self` is a multiple of the divisor, given the precomputed division factor.
    fn is_multiple_of(self, precomputed: Self::PrecomputedDiv) -> bool;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct PrecomputedDivU64 {
    m: u128,
}

impl FastDiv for u64 {
    type PrecomputedDiv = PrecomputedDivU64;

    #[inline]
    fn precompute_div(self) -> Self::PrecomputedDiv {
        //assert!(self > 1);
        Self::PrecomputedDiv {
            m: compute_m_u64(self),
        }
    }

    #[inline]
    fn fast_div(self, precomputed: Self::PrecomputedDiv) -> Self {
        fastdiv_u64(self, precomputed.m)
    }

    #[inline]
    fn fast_mod(self, precomputed: Self::PrecomputedDiv, d: Self) -> Self {
        fastmod_u64(self, precomputed.m, d)
    }

    #[inline]
    fn is_multiple_of(self, precomputed: Self::PrecomputedDiv) -> bool {
        is_divisible_u64(self, precomputed.m)
    }
}

#[inline]
fn integer_decode_f64(f: f64) -> (u64, i16, i8) {
    let bits: u64 = f.to_bits();
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };
    // Exponent bias + mantissa shift
    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}
fn main() {
	//let m=1u64.precompute_div();
    //println!("{:?}",m.m);
    //println!("1000/1={:?}",1000u64.fast_div(m));
    println!("{:?}",mul128_u64(1u128<<64,1000));
}