# Rand

[![Test Status](https://github.com/rust-random/rand/workflows/Tests/badge.svg?event=push)](https://github.com/rust-random/rand/actions)
[![Crate](https://img.shields.io/crates/v/rand.svg)](https://crates.io/crates/rand)
[![Book](https://img.shields.io/badge/book-master-yellow.svg)](https://github.com/mahmudulislam299/rust_math_lib_test)
[![API](https://img.shields.io/badge/api-master-yellow.svg)](https://rust-random.github.io/rand/rand)
[![API](https://docs.rs/rand/badge.svg)](https://docs.rs/rand)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.51+-lightgray.svg)](https://github.com/mahmudulislam299/rust_math_lib_test)

A Rust library for math library test, featuring:

-   This is a test library
-   the output series will always be positive



It's also worth pointing out what `rand` *is not*:

-   Small. Most low-level crates are small, but the higher-level `rand` and
    `rand_distr` each contain a lot of functionality.
-   Simple (implementation). We have a strong focus on correctness, speed and flexibility, but
    not simplicity. If you prefer a small-and-simple library, there are
    alternatives including [fastrand](https://crates.io/crates/fastrand)
    and [oorandom](https://crates.io/crates/oorandom).


Documentation:

-   [API reference (master branch)](https://docs.rs/sqare-series-crate/0.1.0/sqare_series_crate/)
-   [API reference (docs.rs)](https://docs.rs/sqare-series-crate/0.1.0/sqare_series_crate/)


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
math_lib_test = "0.1.0"
```




## Versions

sqare-series-crate is *mature* (suitable for general usage, with infrequent breaking releases
which minimise breakage) but not yet at 1.0. We maintain compatibility with
pinned versions of the Rust compiler (see below).



A detailed [changelog](CHANGELOG.md) is available for releases.

When upgrading to the next minor series (especially 0.4 → 0.5), we recommend
reading the [Upgrade Guide](https://rust-random.github.io/book/update.html).


# License

Rand is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.