use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct OrderedFloat(pub f32);

impl OrderedFloat {
    pub fn into_inner(&self) -> f32 {
        self.0
    }
}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }

    fn le(&self, other: &Self) -> bool {
        other.ge(self)
    }

    fn gt(&self, other: &Self) -> bool {
        !other.ge(self)
    }

    fn ge(&self, other: &Self) -> bool {
        // We consider all NaNs equal, and NaN is the largest possible
        // value. Thus if self is NaN we always return true. Otherwise
        // self >= other is correct. If other is also not NaN it is trivially
        // correct, and if it is we note that nothing can be greater or
        // equal to NaN except NaN itself, which we already handled earlier.
        self.0.is_nan() | (self.0 >= other.0)
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        #[allow(clippy::comparison_chain)]
        if self < other {
            Ordering::Less
        } else if self > other {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() {
            other.0.is_nan()
        } else {
            self.0 == other.0
        }
    }
}

impl Eq for OrderedFloat {}

impl Hash for OrderedFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        const CANONICAL_NAN_BITS: u64 = 0x7ff8000000000000u64;
        fn raw_double_bits(f: f32) -> u64 {
            fn integer_decode_f32(f: f32) -> (u64, i16, i8) {
                let bits: u32 = f.to_bits();
                let sign: i8 = if bits >> 31 == 0 { 1 } else { -1 };
                let mut exponent: i16 = ((bits >> 23) & 0xff) as i16;
                let mantissa = if exponent == 0 {
                    (bits & 0x7fffff) << 1
                } else {
                    (bits & 0x7fffff) | 0x800000
                };
                // Exponent bias + mantissa shift
                exponent -= 127 + 23;
                (mantissa as u64, exponent, sign)
            }

            // masks for the parts of the IEEE 754 float
            const SIGN_MASK: u64 = 0x8000000000000000u64;
            const EXP_MASK: u64 = 0x7ff0000000000000u64;
            const MAN_MASK: u64 = 0x000fffffffffffffu64;
            let (man, exp, sign) = integer_decode_f32(f);
            let exp_u64 = exp as u16 as u64;
            let sign_u64 = (sign > 0) as u64;
            (man & MAN_MASK) | ((exp_u64 << 52) & EXP_MASK) | ((sign_u64 << 63) & SIGN_MASK)
        }
        let bits = if self.0.is_nan() {
            CANONICAL_NAN_BITS
        } else {
            fn canonicalize_signed_zero(x: f32) -> f32 {
                // -0.0 + 0.0 == +0.0 under IEEE754 roundTiesToEven rounding mode,
                // which Rust guarantees. Thus by adding a positive zero we
                // canonicalize signed zero without any branches in one instruction.
                x + 0.0_f32
            }
            raw_double_bits(canonicalize_signed_zero(self.0))
        };

        bits.hash(state)
    }
}

impl Debug for OrderedFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}