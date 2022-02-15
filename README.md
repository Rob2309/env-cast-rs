# env-cast

![MIT license](https://img.shields.io/crates/l/env-cast?style=for-the-badge)
[![crates.io version](https://img.shields.io/crates/v/env-cast?style=for-the-badge)](https://crates.io/crates/env-cast)
[![CI](https://img.shields.io/github/workflow/status/rob2309/env-cast-rs/Continuous%20Integration?label=CI&style=for-the-badge)](https://github.com/Rob2309/env-cast-rs/actions/workflows/ci.yaml)
[![docs](https://img.shields.io/docsrs/env-cast?style=for-the-badge)](https://docs.rs/env-cast)

`env_cast!` reads an environment variable just like `env!("XXX")`, but parses it into a specific type.

Supported types are currently
`i8, u8, i16, u16, i32, u32, i64, u64, f32, f64`.

## Example
```rust
use env_cast::env_cast;
let PKG_MAJOR: u32 = env_cast!("CARGO_PKG_VERSION_MAJOR" as u32);
```
