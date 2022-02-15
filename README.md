# env-cast

`env_cast!` reads an environment variable just like `env!("XXX")`, but parses it into a specific type.

Supported types are currently
`i8, u8, i16, u16, i32, u32, i64, u64, f32, f64`.

## Example
```rust
use env_cast::env_cast;
let PKG_MAJOR: u32 = env_cast!("CARGO_PKG_VERSION_MAJOR" as u32);
```
