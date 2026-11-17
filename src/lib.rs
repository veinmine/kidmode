//! Mord: a native Horizon mod. The launcher compiles this crate against the pinned game build.
//! Each method of `Hooks` is one moment in the game; implement the ones the mod needs
//! (docs/modding/src/start/writing-hooks.md). Need a moment no hook covers? See
//! docs/modding/src/concepts/injections.md.
use mod_api::{self, Hooks};

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {}
