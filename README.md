# non-max

[![GitHub CI Status](https://github.com/LPGhatguy/non-max/workflows/CI/badge.svg)](https://github.com/LPGhatguy/non-max/actions)
[![non-max on crates.io](https://img.shields.io/crates/v/non-max.svg)](https://crates.io/crates/non-max)
[![non-max docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/non-max)

non-max provides types similar to the std `NonZero*` types, but instead requires
that their values are not the maximum for their type. This ensures that
`Option<NonMax*>` is no larger than `NonMax*`.

### Minimum Supported Rust Version (MSRV)

non-max supports Rust 1.34.1 and newer. Until this library reaches 1.0,
changes to the MSRV will require major version bumps. After 1.0, MSRV changes
will only require minor version bumps, but will need significant justification.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
