/*!
[![GitHub CI Status](https://github.com/LPGhatguy/non-max/workflows/CI/badge.svg)](https://github.com/LPGhatguy/non-max/actions)
[![non-max on crates.io](https://img.shields.io/crates/v/non-max.svg)](https://crates.io/crates/non-max)
[![non-max docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/non-max)

non-max provides types similar to the std `NonZero*` types, but instead requires
that their values are not the maximum for their type. This ensures that
`Option<NonMax*>` is no larger than `NonMax*`.

## Minimum Supported Rust Version (MSRV)

non-max supports Rust 1.34.1 and newer. Until this library reaches 1.0,
changes to the MSRV will require major version bumps. After 1.0, MSRV changes
will only require minor version bumps, but will need significant justification.
*/

// This crate is sensitive to integer overflow and wrapping behavior. As such,
// we should usually use methods like `checked_add` and `checked_sub` instead
// of the `Add` or `Sub` operators.
// #![deny(clippy::integer_arithmetic)]
// #![forbid(missing_docs)]

macro_rules! non_max {
    ( $non_max: ident, $non_zero: ident, $primitive: ident ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $non_max(std::num::$non_zero);

        impl $non_max {
            #[inline]
            pub fn new(value: $primitive) -> Option<Self> {
                if value == $primitive::max_value() {
                    None
                } else {
                    let inner = unsafe { std::num::$non_zero::new_unchecked(value + 1) };
                    Some(Self(inner))
                }
            }

            #[inline]
            pub unsafe fn new_unchecked(value: $primitive) -> Self {
                let inner = std::num::$non_zero::new_unchecked(value + 1);
                Self(inner)
            }

            #[inline]
            pub fn get(&self) -> $primitive {
                self.0.get() - 1
            }
        }

        #[cfg(test)]
        mod $primitive {
            use super::*;

            use std::mem::size_of;

            #[test]
            fn construct() {
                let zero = $non_max::new(0).unwrap();
                assert_eq!(zero.get(), 0);

                let some = $non_max::new(19).unwrap();
                assert_eq!(some.get(), 19);

                let max = $non_max::new($primitive::max_value());
                assert_eq!(max, None);
            }

            #[test]
            fn sizes_correct() {
                assert_eq!(size_of::<$primitive>(), size_of::<$non_max>());
                assert_eq!(size_of::<$non_max>(), size_of::<Option<$non_max>>());
            }
        }
    };
}

non_max!(NonMaxI8, NonZeroI8, i8);
non_max!(NonMaxI16, NonZeroI16, i16);
non_max!(NonMaxI32, NonZeroI32, i32);
non_max!(NonMaxI64, NonZeroI64, i64);
non_max!(NonMaxISize, NonZeroIsize, isize);

non_max!(NonMaxU8, NonZeroU8, u8);
non_max!(NonMaxU16, NonZeroU16, u16);
non_max!(NonMaxU32, NonZeroU32, u32);
non_max!(NonMaxU64, NonZeroU64, u64);
non_max!(NonMaxUSize, NonZeroUsize, usize);
