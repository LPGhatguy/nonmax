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
* `NonMaxI128`
* `NonMaxIsize`
* `NonMaxU8`
* `NonMaxU16`
* `NonMaxU32`
* `NonMaxU64`
* `NonMaxU128`
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

## Features

* `std` (default): implements [`std::error::Error`] for [`ParseIntError`] and
[`TryFromIntError`]. Disable this feature for
[`#![no_std]`](https://rust-embedded.github.io/book/intro/no-std.html) support.

## Minimum Supported Rust Version (MSRV)

nonmax supports Rust 1.47.0 and newer. Until this library reaches 1.0,
changes to the MSRV will require major version bumps. After 1.0, MSRV changes
will only require minor version bumps, but will need significant justification.
*/

#![forbid(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

/// An error type returned when a checked integral type conversion fails (mimics [std::num::TryFromIntError])
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryFromIntError(());

#[cfg(feature = "std")]
impl std::error::Error for TryFromIntError {}

impl core::fmt::Display for TryFromIntError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        "out of range integral type conversion attempted".fmt(fmt)
    }
}

impl From<core::num::TryFromIntError> for TryFromIntError {
    fn from(_: core::num::TryFromIntError) -> Self {
        Self(())
    }
}

impl From<core::convert::Infallible> for TryFromIntError {
    fn from(never: core::convert::Infallible) -> Self {
        match never {}
    }
}

/// An error type returned when an integer cannot be parsed (mimics [std::num::ParseIntError])
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseIntError(());

#[cfg(feature = "std")]
impl std::error::Error for ParseIntError {}

impl core::fmt::Display for ParseIntError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        "unable to parse integer".fmt(fmt)
    }
}

impl From<core::num::ParseIntError> for ParseIntError {
    fn from(_: core::num::ParseIntError) -> Self {
        Self(())
    }
}

// error[E0658]: the `!` type is experimental
// https://github.com/rust-lang/rust/issues/35121
// impl From<!> for TryFromIntError { ... }

// https://doc.rust-lang.org/1.47.0/src/core/num/mod.rs.html#31-43
macro_rules! impl_nonmax_fmt {
    ( ( $( $Trait: ident ),+ ) for $nonmax: ident ) => {
        $(
            impl core::fmt::$Trait for $nonmax {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    core::fmt::$Trait::fmt(&self.get(), f)
                }
            }
        )+
    };
}

macro_rules! nonmax {
    ( common, $nonmax: ident, $non_zero: ident, $primitive: ident ) => {
        /// An integer that is known not to equal its maximum value.
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $nonmax(core::num::$non_zero);

        impl $nonmax {
            /// Creates a new non-max if the given value is not the maximum
            /// value.
            #[inline]
            pub const fn new(value: $primitive) -> Option<Self> {
                match core::num::$non_zero::new(value ^ $primitive::MAX) {
                    None => None,
                    Some(value) => Some(Self(value)),
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
                let inner = core::num::$non_zero::new_unchecked(value ^ $primitive::MAX);
                Self(inner)
            }

            /// Returns the value as a primitive type.
            #[inline]
            pub const fn get(&self) -> $primitive {
                self.0.get() ^ $primitive::MAX
            }

            /// Gets non-max with the value zero (0)
            pub const ZERO: $nonmax = unsafe { Self::new_unchecked(0) };

            /// Gets non-max with the value one (1)
            pub const ONE: $nonmax = unsafe { Self::new_unchecked(1) };

            /// Gets non-max with maximum possible value (which is maximum of the underlying primitive minus one)
            pub const MAX: $nonmax = unsafe { Self::new_unchecked($primitive::MAX - 1) };
        }

        impl Default for $nonmax {
            fn default() -> Self {
                unsafe { Self::new_unchecked(0) }
            }
        }

        impl From<$nonmax> for $primitive {
            fn from(value: $nonmax) -> Self {
                value.get()
            }
        }

        impl core::convert::TryFrom<$primitive> for $nonmax {
            type Error = TryFromIntError;
            fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                Self::new(value).ok_or(TryFromIntError(()))
            }
        }

        impl core::str::FromStr for $nonmax {
            type Err = ParseIntError;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new($primitive::from_str(value)?).ok_or(ParseIntError(()))
            }
        }

        impl core::cmp::Ord for $nonmax {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.get().cmp(&other.get())
            }
        }
        impl core::cmp::PartialOrd for $nonmax {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        // NonZero can implement BitOr (will never 0 a nonzero value) but not BitAnd.
        // NonMax can implement BitAnd but not BitOr, with some caveats for signed values:
        // -1 (11...11) & max (01...11) can result in signed max (01...11), so both operands must be nonmax for signed variants

        impl core::ops::BitAnd<$nonmax> for $nonmax {
            type Output = $nonmax;
            fn bitand(self, rhs: $nonmax) -> Self::Output {
                // Safety: since `rhs` is non-max, the result of the
                // bitwise-and will be non-max regardless of the value of `self`
                unsafe { $nonmax::new_unchecked(self.get() & rhs.get()) }
            }
        }

        impl core::ops::BitAndAssign<$nonmax> for $nonmax {
            fn bitand_assign(&mut self, rhs: $nonmax) {
                *self = *self & rhs;
            }
        }

        // https://doc.rust-lang.org/1.47.0/src/core/num/mod.rs.html#173-175
        impl_nonmax_fmt! {
            (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $nonmax
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for $nonmax {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.get().serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $nonmax {
            fn deserialize<D>(deserializer: D) -> Result<$nonmax, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = $primitive::deserialize(deserializer)?;
                use core::convert::TryFrom;
                Self::try_from(value).map_err(serde::de::Error::custom)
            }
        }

        #[cfg(test)]
        mod $primitive {
            use super::*;

            use core::mem::size_of;

            #[test]
            fn construct() {
                let zero = $nonmax::new(0).unwrap();
                assert_eq!(zero.get(), 0);

                let some = $nonmax::new(19).unwrap();
                assert_eq!(some.get(), 19);

                let max = $nonmax::new($primitive::MAX);
                assert_eq!(max, None);
            }

            #[test]
            fn sizes_correct() {
                assert_eq!(size_of::<$primitive>(), size_of::<$nonmax>());
                assert_eq!(size_of::<$nonmax>(), size_of::<Option<$nonmax>>());
            }

            #[test]
            fn convert() {
                use core::convert::TryFrom;
                let zero = $nonmax::try_from(0 as $primitive).unwrap();
                let zero = $primitive::from(zero);
                assert_eq!(zero, 0);

                $nonmax::try_from($primitive::MAX).unwrap_err();
            }

            #[test]
            fn cmp() {
                let zero = $nonmax::new(0).unwrap();
                let one = $nonmax::new(1).unwrap();
                let two = $nonmax::new(2).unwrap();
                assert!(zero < one);
                assert!(one < two);
                assert!(two > one);
                assert!(one > zero);
            }

            #[test]
            fn constants() {
                let zero = $nonmax::ZERO;
                let one = $nonmax::ONE;
                let max = $nonmax::MAX;
                assert_eq!(zero.get(), 0);
                assert_eq!(one.get(), 1);
                assert_eq!(max.get(), $primitive::MAX - 1);
            }

            #[test]
            #[cfg(feature = "std")] // to_string
            fn parse() {
                for value in [0, 19, $primitive::MAX - 1].iter().copied() {
                    let string = value.to_string();
                    let nonmax = string.parse::<$nonmax>().unwrap();
                    assert_eq!(nonmax.get(), value);
                }
                $primitive::MAX.to_string().parse::<$nonmax>().unwrap_err();
            }

            #[test]
            #[cfg(feature = "std")] // format!
            fn fmt() {
                let zero = $nonmax::new(0).unwrap();
                let some = $nonmax::new(19).unwrap();
                let max1 = $nonmax::new($primitive::MAX - 1).unwrap();
                for value in [zero, some, max1].iter().copied() {
                    assert_eq!(format!("{}", value.get()), format!("{}", value)); // Display
                    assert_eq!(format!("{:?}", value.get()), format!("{:?}", value)); // Debug
                    assert_eq!(format!("{:b}", value.get()), format!("{:b}", value)); // Binary
                    assert_eq!(format!("{:o}", value.get()), format!("{:o}", value)); // Octal
                    assert_eq!(format!("{:x}", value.get()), format!("{:x}", value)); // LowerHex
                    assert_eq!(format!("{:X}", value.get()), format!("{:X}", value)); // UpperHex
                }
            }

            #[test]
            #[cfg(feature = "serde")]
            fn serde() {
                for &value in [0, 19, $primitive::MAX - 1].iter() {
                    let nonmax_value = $nonmax::new(value).unwrap();
                    let encoded: Vec<u8> = bincode::serialize(&nonmax_value).unwrap();
                    let decoded: $nonmax = bincode::deserialize(&encoded[..]).unwrap();
                    assert_eq!(nonmax_value, decoded);
                }
            }
        }
    };

    ( signed, $nonmax: ident, $non_zero: ident, $primitive: ident ) => {
        nonmax!(common, $nonmax, $non_zero, $primitive);
        // Nothing unique to signed versions (yet)
    };

    ( unsigned, $nonmax: ident, $non_zero: ident, $primitive: ident ) => {
        nonmax!(common, $nonmax, $non_zero, $primitive);

        impl core::ops::BitAnd<$nonmax> for $primitive {
            type Output = $nonmax;
            fn bitand(self, rhs: $nonmax) -> Self::Output {
                // Safety: since `rhs` is non-max, the result of the
                // bitwise-and will be non-max regardless of the value of `self`
                unsafe { $nonmax::new_unchecked(self & rhs.get()) }
            }
        }

        impl core::ops::BitAnd<$primitive> for $nonmax {
            type Output = $nonmax;
            fn bitand(self, rhs: $primitive) -> Self::Output {
                // Safety: since `self` is non-max, the result of the
                // bitwise-and will be non-max regardless of the value of `rhs`
                unsafe { $nonmax::new_unchecked(self.get() & rhs) }
            }
        }

        impl core::ops::BitAndAssign<$primitive> for $nonmax {
            fn bitand_assign(&mut self, rhs: $primitive) {
                *self = *self & rhs;
            }
        }

        // std doesn't have an equivalent BitAndOr for $nonzero, but this just makes sense
        impl core::ops::BitAndAssign<$nonmax> for $primitive {
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

// https://doc.rust-lang.org/1.47.0/src/core/convert/num.rs.html#383-407
macro_rules! impl_nonmax_from {
    ( $small: ty, $large: ty ) => {
        impl From<$small> for $large {
            #[inline]
            fn from(small: $small) -> Self {
                // SAFETY: smaller input type guarantees the value is non-max
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

// https://doc.rust-lang.org/1.47.0/src/core/convert/num.rs.html#383-407
macro_rules! impl_smaller_from {
    ( $small: ty, $large: ty ) => {
        impl From<$small> for $large {
            #[inline]
            fn from(small: $small) -> Self {
                // SAFETY: smaller input type guarantees the value is non-max
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

#[cfg(test)]
mod ops {
    use super::*;

    #[test]
    fn bitand_unsigned() {
        for left in 0..=u8::MAX {
            let nmleft = NonMaxU8::new(left);
            for right in 0..=u8::MAX {
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
        for left in i8::MIN..=i8::MAX {
            let nmleft = NonMaxI8::new(left);
            for right in i8::MIN..=i8::MAX {
                let nmright = NonMaxI8::new(right);
                let vanilla = left & right;
                if let (Some(nmleft), Some(nmright)) = (nmleft, nmright) {
                    assert_eq!(vanilla, (nmleft & nmright).get());
                }
            }
        }
    }
}
