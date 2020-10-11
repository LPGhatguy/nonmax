# nonmax Changelog

## Unreleased Changes
* Raised MSRV to 1.47.0 to reduce `unsafe` by using `NonZero*::new`
* Implement a bunch of traits that NonZero had but NonMax was missing
* Added `NonMaxI128` and `NonMaxU128` support

## 0.4.0 (2020-09-27)
* Raised MSRV to 1.46.0 to make more methods `const`.
* Marked all methods as `const`.
* Added documentation for all items.

## 0.3.0 (2020-09-27)
* Fixed casing of `NonMaxUsize` and `NonMaxIsize`

## 0.2.0 (2020-09-27)
* Initial release