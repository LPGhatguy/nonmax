# nonmax Changelog

## Unreleased Changes

## 0.5.5 (2023-11-19)
* Fixed `serde` feature not working in `no_std` contexts ([#16])

[#16]: https://github.com/LPGhatguy/nonmax/pull/16

## 0.5.4 (2023-11-09)
* Added `ZERO`, `ONE`, and `MAX` associated constants. ([#15])

[#15]: https://github.com/LPGhatguy/nonmax/pull/15

## 0.5.3 (2022-07-01)
* Fixed `PartialOrd` and `Ord` implementations being backwards.

## 0.5.2 (20222-07-01)
(bad release, yanked)

## 0.5.1 (2022-06-29)
* Implemented `Default` for `NonMax*`.
* Added serialization/deserialization with `serde` when the corresponding feature is enabled.

## 0.5.0 (2020-10-18)
* Raised MSRV to 1.47.0 to eliminate `unsafe` in `new`.
* Added `NonMaxI128` and `NonMaxU128`.
* Implemented `std::convert::[Try]From` for `NonMax*` and primitive types to match `NonZero*`.
* Implemented `std::fmt::{Display, Binary, Octal, LowerHex, UpperHex}` to match `NonZero*`.
* Removed outer "NonMax(...)" in output from `impl Debug for NonMax*`, to match `NonZero*`.
* Implemented `std::str::FromStr` for `NonMax*` to match `NonZero*`.
* Implemented `std::ops::BitAnd[Assign]` for `NonMax*`, similar to `BitOr[Assign]` for `NonZero*`.
* Added `no_std` support.

## 0.4.0 (2020-09-27)
* Raised MSRV to 1.46.0 to make more methods `const`.
* Marked all methods as `const`.
* Added documentation for all items.

## 0.3.0 (2020-09-27)
* Fixed casing of `NonMaxUsize` and `NonMaxIsize` to match `NonZero*`.

## 0.2.0 (2020-09-27)
* Initial release
