# nonmax Changelog

## Unreleased Changes
* Added `NonMaxI128` and `NonMaxU128` support
* Implemented `std::convert::[Try]From` for `NonMax*` & primitive types to match `NonZero*`
* Implemented `std::fmt::{Display, Binary, Octal, LowerHex, UpperHex}` to match `NonZero*`
* Removed outer "NonMax(...)" in output from `impl Debug for NonMax*`, matching `NonZero*` behavior
* Implemented `std::str::FromStr` for `NonMax*` to match `NonZero*`
* Implemented `std::ops::BitAnd[Assign]` for `NonMax*`, similar to `BitOr[Assign]` for `NonZero*`

## 0.4.0 (2020-09-27)
* Raised MSRV to 1.46.0 to make more methods `const`.
* Marked all methods as `const`.
* Added documentation for all items.

## 0.3.0 (2020-09-27)
* Fixed casing of `NonMaxUsize` and `NonMaxIsize`

## 0.2.0 (2020-09-27)
* Initial release