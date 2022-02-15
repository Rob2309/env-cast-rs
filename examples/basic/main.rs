use env_cast::env_cast;

const VERSION_MAJOR: u32 = env_cast!("CARGO_PKG_VERSION_MAJOR" as u32);
const VERSION_MINOR: u32 = env_cast!("CARGO_PKG_VERSION_MINOR" as u32);
const VERSION_PATCH: u32 = env_cast!("CARGO_PKG_VERSION_PATCH" as u32);

fn main() {
    println!(
        "env-cast v{}.{}.{}",
        VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH
    );
}
