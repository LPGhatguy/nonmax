# nonmax Changelog

## Unreleased Changes
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