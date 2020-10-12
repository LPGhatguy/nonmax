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

## Minimum Supported Rust Version (MSRV)

nonmax supports Rust 1.46.0 and newer. Until this library reaches 1.0,
changes to the MSRV will require major version bumps. After 1.0, MSRV changes
will only require minor version bumps, but will need significant justification.
*/

#![forbid(missing_docs)]

macro_rules! nonmax {
    ( $nonmax: ident, $non_zero: ident, $primitive: ident ) => {
        /// An integer that is known not to equal its maximum value.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $nonmax(std::num::$non_zero);

        impl $nonmax {
            /// Creates a new non-max if the given value is not the maximum
            /// value.
            #[inline]
            pub const fn new(value: $primitive) -> Option<Self> {
                if value == $primitive::max_value() {
                    None
                } else {
                    let inner = unsafe {
                        std::num::$non_zero::new_unchecked(value ^ $primitive::max_value())
                    };
                    Some(Self(inner))
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

        impl std::convert::TryFrom<$primitive> for $nonmax {
            type Error = std::num::TryFromIntError;
            fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                Self::new(value).ok_or_else(try_from_int_error_overflow)
            }
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
            fn convert() {
                use std::convert::TryFrom;
                let zero = $nonmax::try_from(0 as $primitive).unwrap();
                let zero = $primitive::from(zero);
                assert_eq!(zero, 0);

                $nonmax::try_from($primitive::max_value()).unwrap_err();
            }
        }
    };
}

fn try_from_int_error_overflow() -> std::num::TryFromIntError {
    // Why yes, we *do* need to resort to this ugliness just to create a TryFromIntError!
    use std::convert::TryFrom;
    u8::try_from(999u32).unwrap_err() // infalliable
}

nonmax!(NonMaxI8, NonZeroI8, i8);
nonmax!(NonMaxI16, NonZeroI16, i16);
nonmax!(NonMaxI32, NonZeroI32, i32);
nonmax!(NonMaxI64, NonZeroI64, i64);
nonmax!(NonMaxI128, NonZeroI128, i128);
nonmax!(NonMaxIsize, NonZeroIsize, isize);

nonmax!(NonMaxU8, NonZeroU8, u8);
nonmax!(NonMaxU16, NonZeroU16, u16);
nonmax!(NonMaxU32, NonZeroU32, u32);
nonmax!(NonMaxU64, NonZeroU64, u64);
nonmax!(NonMaxU128, NonZeroU128, u128);
nonmax!(NonMaxUsize, NonZeroUsize, usize);

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
