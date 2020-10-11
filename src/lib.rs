/*!
[![GitHub CI Status](https://github.com/LPGhatguy/nonmax/workflows/CI/badge.svg)](https://github.com/LPGhatguy/nonmax/actions)
[![nonmax on crates.io](https://img.shields.io/crates/v/nonmax.svg)](https://crates.io/crates/nonmax)
[![nonmax docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/nonmax)

nonmax provides types similar to the std `NonZero*` types, but instead requires
that their values are not the maximum for their type. This ensures that
`Option<NonMax*>` is no larger than `NonMax*`.

nonmax supports every type that has a corresponding non-zero variant in the
standard library:

* `NonMaxI8`
* `NonMaxI16`
* `NonMaxI32`
* `NonMaxI64`
* `NonMaxIsize`
* `NonMaxU8`
* `NonMaxU16`
* `NonMaxU32`
* `NonMaxU64`
* `NonMaxUsize`

## Example

```
use nonmax::{NonMaxI16, NonMaxU8};

let value = NonMaxU8::new(16).expect("16 should definitely fit in a u8");
assert_eq!(value.get(), 16);
assert_eq!(std::mem::size_of_val(&value), 1);

let signed = NonMaxI16::new(i16::min_value()).expect("minimum values are fine");
assert_eq!(signed.get(), i16::min_value());
assert_eq!(std::mem::size_of_val(&signed), 2);

let oops = NonMaxU8::new(255);
assert_eq!(oops, None);
```

## Minimum Supported Rust Version (MSRV)

nonmax supports Rust 1.47.0 and newer. Until this library reaches 1.0,
changes to the MSRV will require major version bumps. After 1.0, MSRV changes
will only require minor version bumps, but will need significant justification.
*/

#![forbid(missing_docs)]

// https://doc.rust-lang.org/1.47.0/src/core/num/mod.rs.html#31-43
macro_rules! impl_nonmax_fmt {
    ( ( $( $Trait: ident ),+ ) for $nonmax: ident ) => {
        $(
            impl std::fmt::$Trait for $nonmax {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::$Trait::fmt(&self.get(), f)
                }
            }
        )+
    };
}

macro_rules! nonmax {
    ( common, $nonmax: ident, $non_zero: ident, $primitive: ident ) => {
        /// An integer that is known not to equal its maximum value.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $nonmax(std::num::$non_zero);

        impl $nonmax {
            /// Creates a new non-max if the given value is not the maximum
            /// value.
            #[inline]
            pub const fn new(value: $primitive) -> Option<Self> {
                if let Some(v) = std::num::$non_zero::new(value ^ $primitive::max_value()) {
                    Some(Self(v))
                } else {
                    None
                }
            }

            /// Creates a new non-max without checking the value.
            ///
            /// # Safety
            ///
            /// The value must not equal the maximum representable value for the
            /// primitive type.
            #[inline]
            pub const unsafe fn new_unchecked(value: $primitive) -> Self {
                let inner = std::num::$non_zero::new_unchecked(value ^ $primitive::max_value());
                Self(inner)
            }

            /// Returns the value as a primitive type.
            #[inline]
            pub const fn get(&self) -> $primitive {
                self.0.get() ^ $primitive::max_value()
            }
        }

        impl From<$nonmax> for $primitive {
            fn from(value: $nonmax) -> Self {
                value.get()
            }
        }

        impl std::str::FromStr for $nonmax {
            type Err = std::num::ParseIntError;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new($primitive::from_str(value)?).ok_or_else(parse_int_error_overflow)
            }
        }

        impl std::convert::TryFrom<$primitive> for $nonmax {
            type Error = std::num::TryFromIntError;
            fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                Self::new(value).ok_or_else(try_from_int_error_overflow)
            }
        }

        // NonZero can implement BitOr (will never 0 a nonzero value) but not BitAnd.
        // NonMax can implement BitAnd but not BitOr, with some caveats for signed values:
        // -1 (11...11) & max (01...11) can result in signed max (01...11), so both operands must be nonmax for signed variants

        impl std::ops::BitAnd<$nonmax> for $nonmax {
            type Output = $nonmax;
            fn bitand(self, rhs: $nonmax) -> Self::Output {
                // Safety: since `rhs` is non-max, the result of the
                // bitwise-and will be non-max regardless of the value of `self`
                unsafe { $nonmax::new_unchecked(self.get() & rhs.get()) }
            }
        }

        impl std::ops::BitAndAssign<$nonmax> for $nonmax {
            fn bitand_assign(&mut self, rhs: $nonmax) {
                *self = *self & rhs;
            }
        }

        impl_nonmax_fmt! {
            (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $nonmax
        }

        #[cfg(test)]
        mod $primitive {
            use super::*;

            use std::mem::size_of;

            #[test]
            fn construct() {
                let zero = $nonmax::new(0).unwrap();
                assert_eq!(zero.get(), 0);

                let some = $nonmax::new(19).unwrap();
                assert_eq!(some.get(), 19);

                let max = $nonmax::new($primitive::max_value());
                assert_eq!(max, None);
            }

            #[test]
            fn sizes_correct() {
                assert_eq!(size_of::<$primitive>(), size_of::<$nonmax>());
                assert_eq!(size_of::<$nonmax>(), size_of::<Option<$nonmax>>());
            }

            #[test]
            fn fmt() {
                let zero = $nonmax::new(0).unwrap();
                let some = $nonmax::new(19).unwrap();
                let max1 = $nonmax::new($primitive::max_value() - 1).unwrap();
                for value in [zero, some, max1].iter().copied() {
                    assert_eq!(format!("{}",   value.get()), format!("{}",   value)); // Display
                    assert_eq!(format!("{:?}", value.get()), format!("{:?}", value)); // Debug
                    assert_eq!(format!("{:b}", value.get()), format!("{:b}", value)); // Binary
                    assert_eq!(format!("{:o}", value.get()), format!("{:o}", value)); // Octal
                    assert_eq!(format!("{:x}", value.get()), format!("{:x}", value)); // LowerHex
                    assert_eq!(format!("{:X}", value.get()), format!("{:X}", value)); // UpperHex
                }
            }

            #[test]
            fn convert() {
                use std::convert::TryFrom;
                let zero = $nonmax::try_from(0 as $primitive).unwrap();
                let zero = $primitive::from(zero);
                assert_eq!(zero, 0);

                use std::str::FromStr;
                let zero = $nonmax::from_str("0").unwrap();
                let zero = $primitive::from(zero);
                assert_eq!(zero, 0);

                $nonmax::try_from($primitive::max_value()).unwrap_err();
                $nonmax::from_str(&$primitive::max_value().to_string()).unwrap_err();
            }
        }
    };

    ( signed, $nonmax: ident, $non_zero: ident, $primitive: ident ) => {
        nonmax!(common, $nonmax, $non_zero, $primitive);
        // Nothing unique to signed versions (yet)
    };

    ( unsigned, $nonmax: ident, $non_zero: ident, $primitive: ident ) => {
        nonmax!(common, $nonmax, $non_zero, $primitive);

        impl std::ops::BitAnd<$nonmax> for $primitive {
            type Output = $nonmax;
            fn bitand(self, rhs: $nonmax) -> Self::Output {
                // Safety: since `rhs` is non-max, the result of the
                // bitwise-and will be non-max regardless of the value of `self`
                unsafe { $nonmax::new_unchecked(self & rhs.get()) }
            }
        }

        impl std::ops::BitAnd<$primitive> for $nonmax {
            type Output = $nonmax;
            fn bitand(self, rhs: $primitive) -> Self::Output {
                // Safety: since `self` is non-max, the result of the
                // bitwise-and will be non-max regardless of the value of `rhs`
                unsafe { $nonmax::new_unchecked(self.get() & rhs) }
            }
        }

        impl std::ops::BitAndAssign<$primitive> for $nonmax {
            fn bitand_assign(&mut self, rhs: $primitive) {
                *self = *self & rhs;
            }
        }

        // std doesn't have an equivalent BitAndOr for $nonzero, but this just makes sense
        impl std::ops::BitAndAssign<$nonmax> for $primitive {
            fn bitand_assign(&mut self, rhs: $nonmax) {
                *self = *self & rhs.get();
            }
        }
    };
}

nonmax!(signed, NonMaxI8, NonZeroI8, i8);
nonmax!(signed, NonMaxI16, NonZeroI16, i16);
nonmax!(signed, NonMaxI32, NonZeroI32, i32);
nonmax!(signed, NonMaxI64, NonZeroI64, i64);
nonmax!(signed, NonMaxI128, NonZeroI128, i128);
nonmax!(signed, NonMaxIsize, NonZeroIsize, isize);

nonmax!(unsigned, NonMaxU8, NonZeroU8, u8);
nonmax!(unsigned, NonMaxU16, NonZeroU16, u16);
nonmax!(unsigned, NonMaxU32, NonZeroU32, u32);
nonmax!(unsigned, NonMaxU64, NonZeroU64, u64);
nonmax!(unsigned, NonMaxU128, NonZeroU128, u128);
nonmax!(unsigned, NonMaxUsize, NonZeroUsize, usize);

// https://doc.rust-lang.org/stable/src/core/convert/num.rs.html#383-407
macro_rules! impl_nonmax_from {
    ( $small: ty, $large: ty ) => {
        impl From<$small> for $large {
            #[inline]
            fn from(small: $small) -> Self {
                // SAFETY: smaller input type guarantees the value is non-zero
                unsafe { Self::new_unchecked(small.get().into()) }
            }
        }
    };
}

// Non-max Unsigned -> Non-max Unsigned
impl_nonmax_from!(NonMaxU8, NonMaxU16);
impl_nonmax_from!(NonMaxU8, NonMaxU32);
impl_nonmax_from!(NonMaxU8, NonMaxU64);
impl_nonmax_from!(NonMaxU8, NonMaxU128);
impl_nonmax_from!(NonMaxU8, NonMaxUsize);
impl_nonmax_from!(NonMaxU16, NonMaxU32);
impl_nonmax_from!(NonMaxU16, NonMaxU64);
impl_nonmax_from!(NonMaxU16, NonMaxU128);
impl_nonmax_from!(NonMaxU16, NonMaxUsize);
impl_nonmax_from!(NonMaxU32, NonMaxU64);
impl_nonmax_from!(NonMaxU32, NonMaxU128);
impl_nonmax_from!(NonMaxU64, NonMaxU128);

// Non-max Signed -> Non-max Signed
impl_nonmax_from!(NonMaxI8, NonMaxI16);
impl_nonmax_from!(NonMaxI8, NonMaxI32);
impl_nonmax_from!(NonMaxI8, NonMaxI64);
impl_nonmax_from!(NonMaxI8, NonMaxI128);
impl_nonmax_from!(NonMaxI8, NonMaxIsize);
impl_nonmax_from!(NonMaxI16, NonMaxI32);
impl_nonmax_from!(NonMaxI16, NonMaxI64);
impl_nonmax_from!(NonMaxI16, NonMaxI128);
impl_nonmax_from!(NonMaxI16, NonMaxIsize);
impl_nonmax_from!(NonMaxI32, NonMaxI64);
impl_nonmax_from!(NonMaxI32, NonMaxI128);
impl_nonmax_from!(NonMaxI64, NonMaxI128);

// Non-max Unsigned -> Non-max Signed
impl_nonmax_from!(NonMaxU8, NonMaxI16);
impl_nonmax_from!(NonMaxU8, NonMaxI32);
impl_nonmax_from!(NonMaxU8, NonMaxI64);
impl_nonmax_from!(NonMaxU8, NonMaxI128);
impl_nonmax_from!(NonMaxU8, NonMaxIsize);
impl_nonmax_from!(NonMaxU16, NonMaxI32);
impl_nonmax_from!(NonMaxU16, NonMaxI64);
impl_nonmax_from!(NonMaxU16, NonMaxI128);
impl_nonmax_from!(NonMaxU32, NonMaxI64);
impl_nonmax_from!(NonMaxU32, NonMaxI128);
impl_nonmax_from!(NonMaxU64, NonMaxI128);

// https://doc.rust-lang.org/stable/src/core/convert/num.rs.html#383-407
macro_rules! impl_smaller_from {
    ( $small: ty, $large: ty ) => {
        impl From<$small> for $large {
            #[inline]
            fn from(small: $small) -> Self {
                // SAFETY: smaller input type guarantees the value is non-zero
                unsafe { Self::new_unchecked(small.into()) }
            }
        }
    };
}

// Unsigned -> Non-max Unsigned
impl_smaller_from!(u8, NonMaxU16);
impl_smaller_from!(u8, NonMaxU32);
impl_smaller_from!(u8, NonMaxU64);
impl_smaller_from!(u8, NonMaxU128);
impl_smaller_from!(u8, NonMaxUsize);
impl_smaller_from!(u16, NonMaxU32);
impl_smaller_from!(u16, NonMaxU64);
impl_smaller_from!(u16, NonMaxU128);
impl_smaller_from!(u16, NonMaxUsize);
impl_smaller_from!(u32, NonMaxU64);
impl_smaller_from!(u32, NonMaxU128);
impl_smaller_from!(u64, NonMaxU128);

// Signed -> Non-max Signed
impl_smaller_from!(i8, NonMaxI16);
impl_smaller_from!(i8, NonMaxI32);
impl_smaller_from!(i8, NonMaxI64);
impl_smaller_from!(i8, NonMaxI128);
impl_smaller_from!(i8, NonMaxIsize);
impl_smaller_from!(i16, NonMaxI32);
impl_smaller_from!(i16, NonMaxI64);
impl_smaller_from!(i16, NonMaxI128);
impl_smaller_from!(i16, NonMaxIsize);
impl_smaller_from!(i32, NonMaxI64);
impl_smaller_from!(i32, NonMaxI128);
impl_smaller_from!(i64, NonMaxI128);

// Unsigned -> Non-max Signed
impl_smaller_from!(u8, NonMaxI16);
impl_smaller_from!(u8, NonMaxI32);
impl_smaller_from!(u8, NonMaxI64);
impl_smaller_from!(u8, NonMaxI128);
impl_smaller_from!(u8, NonMaxIsize);
impl_smaller_from!(u16, NonMaxI32);
impl_smaller_from!(u16, NonMaxI64);
impl_smaller_from!(u16, NonMaxI128);
impl_smaller_from!(u32, NonMaxI64);
impl_smaller_from!(u32, NonMaxI128);
impl_smaller_from!(u64, NonMaxI128);

fn parse_int_error_overflow() -> std::num::ParseIntError {
    use std::str::FromStr;
    u8::from_str("999").unwrap_err()
}

fn try_from_int_error_overflow() -> std::num::TryFromIntError {
    use std::convert::TryFrom;
    u8::try_from(999u32).unwrap_err()
}


#[cfg(test)]
mod ops {
    use super::*;

    #[test]
    fn bitand_unsigned() {
        for left in 0 ..= 255 {
            let nmleft = NonMaxU8::new(left);
            for right in 0 ..= 255 {
                let nmright = NonMaxU8::new(right);
                let vanilla = left & right;

                if let (Some(nmleft), Some(nmright)) = (nmleft, nmright) {
                    assert_eq!(vanilla, (nmleft & nmright).get());
                }
                if let Some(nmleft) = nmleft {
                    assert_eq!(vanilla, (nmleft & right).get());
                }
                if let Some(nmright) = nmright {
                    assert_eq!(vanilla, (left & nmright).get());
                }
            }
        }
    }

    #[test]
    fn bitand_signed() {
        for left in -128 ..= 127 {
            let nmleft = NonMaxI8::new(left);
            for right in -128 ..= 127 {
                let nmright = NonMaxI8::new(right);
                let vanilla = left & right;
                if let (Some(nmleft), Some(nmright)) = (nmleft, nmright) {
                    assert_eq!(vanilla, (nmleft & nmright).get());
                }
            }
        }
    }
}
