# nonmax Changelog

## Unreleased Changes
* Added `NonMaxI128` and `NonMaxU128` support
* Implemented `std::convert::[Try]From` for `NonMax*` & primitive types to match `NonZero*`

## 0.4.0 (2020-09-27)
* Raised MSRV to 1.46.0 to make more methods `const`.
* Marked all methods as `const`.
* Added documentation for all items.

## 0.3.0 (2020-09-27)
* Fixed casing of `NonMaxUsize` and `NonMaxIsize`

## 0.2.0 (2020-09-27)
* Initial release