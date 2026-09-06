# Mirl Extensions Math (0.0.0-alpha)

> This is a sub-crate of [Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) ([crates.io](https://crates.io/crates/mirl_extensions))

### Mextma - Numbers, calculations, and code deduplication

[![Version](https://img.shields.io/crates/v/mirl_extensions_math.svg)](https://crates.io/crates/mirl_extensions_math)
[![Documentation](https://docs.rs/mirl_extensions_math/badge.svg)](https://docs.rs/mirl_extensions_math)
[![dependency status](https://deps.rs/repo/github/Miner3D-Gamer/mirl_extensions_math/status.svg)](https://deps.rs/repo/github/Miner3D-Gamer/mirl_extensions_math)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.99–nightly-blue)](https://github.com/rust-lang/rust/releases/tag/1.99.0)
[![Changelog](https://img.shields.io/badge/CHANGELOG.md--555.svg)](https://github.com/Miner3D-Gamer/mirl_extensions_math/blob/main/CHANGELOG.md)

<details>
<summary>Flags</summary>

### Default

**Core**

- `std` (Default)
  ~~- `c_compatible`~~ Only traits and impls are defined
- `all`

**Custom**
TODO

</details>

### Entry Points

- `use mirl_extensions_math::*;`

### Purpose

Let's imagine you create a function that for its calculation needs the sign, abs, and add functions for a struct(/primitive); how would you go about implementing it for every type?

- Write it out manually? Duplicating the code 16 times is tedious, not good for maintenance or readability.

- Write a macro that generates a version for each type? That's an extra macro to write and take care of. Not to mention types outside the crate aren't supported.

- Auto impl the trait/take any generic input with any type that impl `.sign()`, and `.abs()` functions? Now that's a solution.

You could use `num-traits` for this yet that crate has no nightly support.
This lib offers all traits which `num-traits` offers with the difference that it uses nightly features to be even more dynamic and easy to use.

### Origin

Mirl Extensions simply became too big so a few isolated concepts had to be split. Math was one of them.
