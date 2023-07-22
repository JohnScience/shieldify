# Shieldify

[![Crates.io](https://img.shields.io/crates/v/shieldify)](https://crates.io/crates/shieldify)
[![Downloads](https://img.shields.io/crates/d/shieldify.svg)](https://crates.io/crates/shieldify)
[![Documentation](https://docs.rs/shieldify/badge.svg)](https://docs.rs/shieldify)
[![License](https://img.shields.io/crates/l/shieldify)](https://crates.io/crates/shieldify)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/shieldify/status.svg)](https://deps.rs/repo/github/JohnScience/shieldify)

Add sheilds to your README.md with ease.

## Installation

```console
cargo install shieldify
```

## Usage

Add [`repository` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-repository-field) to your `Cargo.toml` and run in the crate root:

```console
shieldify
```

Your `README.md` will be updated with the badges.

## Notes

At the moment of writing, the crate expects that the repository is hosted on GitHub.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
