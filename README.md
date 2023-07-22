# Shieldify

[![Crates.io](https://img.shields.io/crates/v/shieldify)](https://crates.io/crates/shieldify)
[![Downloads](https://img.shields.io/crates/d/shieldify.svg)](https://crates.io/crates/shieldify)
[![License](https://img.shields.io/crates/l/shieldify)](https://crates.io/crates/shieldify)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/shieldify/status.svg)](https://deps.rs/repo/github/JohnScience/shieldify)

Add sheilds (=badges) to your [`README.md`](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-readmes) with ease.

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

For crates with `src/lib.rs`, it will also add a badge for the documentation.

At the moment of writing, the crate expects that the repository is hosted on GitHub.

In theory, the crate might eventually be able to use [`git2`](https://crates.io/crates/git2) to get the repository information from the local repository, but it is not implemented yet.

## Related crates

* [`badge-maker`](https://crates.io/crates/badge-maker)

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
