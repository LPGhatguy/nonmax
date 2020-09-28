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
* `NonMaxISize`
* `NonMaxU8`
* `NonMaxU16`
* `NonMaxU32`
* `NonMaxU64`
* `NonMaxUSize`

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

nonmax supports Rust 1.34.1 and newer. Until this library reaches 1.0,
changes to the MSRV will require major version bumps. After 1.0, MSRV changes
will only require minor version bumps, but will need significant justification.
*/

// This crate is sensitive to integer overflow and wrapping behavior. As such,
// we should usually use methods like `checked_add` and `checked_sub` instead
// of the `Add` or `Sub` operators.
// #![deny(clippy::integer_arithmetic)]
// #![forbid(missing_docs)]

macro_rules! nonmax {
    ( $nonmax: ident, $non_zero: ident, $primitive: ident ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $nonmax(std::num::$non_zero);

        impl $nonmax {
            #[inline]
            pub fn new(value: $primitive) -> Option<Self> {
                if value == $primitive::max_value() {
                    None
                } else {
                    let inner = unsafe {
                        std::num::$non_zero::new_unchecked(value ^ $primitive::max_value())
                    };
                    Some(Self(inner))
                }
            }

            #[inline]
            pub unsafe fn new_unchecked(value: $primitive) -> Self {
                let inner = std::num::$non_zero::new_unchecked(value ^ $primitive::max_value());
                Self(inner)
            }

            #[inline]
            pub fn get(&self) -> $primitive {
                self.0.get() ^ $primitive::max_value()
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
        }
    };
}

nonmax!(NonMaxI8, NonZeroI8, i8);
nonmax!(NonMaxI16, NonZeroI16, i16);
nonmax!(NonMaxI32, NonZeroI32, i32);
nonmax!(NonMaxI64, NonZeroI64, i64);
nonmax!(NonMaxISize, NonZeroIsize, isize);

nonmax!(NonMaxU8, NonZeroU8, u8);
nonmax!(NonMaxU16, NonZeroU16, u16);
nonmax!(NonMaxU32, NonZeroU32, u32);
nonmax!(NonMaxU64, NonZeroU64, u64);
nonmax!(NonMaxUSize, NonZeroUsize, usize);
