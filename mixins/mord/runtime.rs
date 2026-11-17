// Mord, a native Horizon mod by veinmine. This file is both the mod's Cargo
// library (its tests: `cargo test`) and the module the bundle adds to the game
// as `crate::game::mord`. Module payloads cannot use inner attributes or `//!`
// comments, external `mod` files or `include!`.

/// The mod's id, name and version, as Options > Mods shows them.
pub const ID: &str = "mord";
pub const NAME: &str = "Mord";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "An empty native mod, ready to grow.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_lowercase_ascii() {
        assert!(ID.bytes().all(|b| b.is_ascii_lowercase() || b == b'_'));
    }
}
