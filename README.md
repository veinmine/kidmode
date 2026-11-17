# Kid Mode

Kid Mode (repository and mod id `mord`) is a native [Horizon](https://playhorizon.dev) mod by veinmine, in the
horizon-mixins format (`mixins/mord/horizon-mixin.toml`).

```sh
cargo test                                                           # the mod's own logic
horizon-launcher native-build mixins/mord --trust-native-code        # build the game with it, for the selected instance
```

`mixins/mord/runtime.rs` is both the Cargo library and the module added to the
game as `crate::game::mord`; `list.rs` hooks the native `native_mods` point to
list the mod in Options > Mods. See https://playhorizon.dev/docs/make-a-mod.

Features and their settings are listed in [docs/features.md](docs/features.md).
