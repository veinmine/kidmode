# Kid Mode

Kid Mode (repository and mod id `kidmode`) is a native [Horizon](https://playhorizon.dev) mod by veinmine, in the
horizon-mixins format (`mixins/kidmode/horizon-mixin.toml`).

```sh
cargo test                                                           # the mod's own logic
horizon-launcher native-build mixins/kidmode --trust-native-code     # build the game with it, for the selected instance
```

`mixins/kidmode/runtime.rs` is both the Cargo library and the module added to the
game as `crate::game::kidmode`; `list.rs` hooks the native `native_mods` point to
list the mod in Options > Mods. See https://playhorizon.dev/docs/make-a-mod.

Features and their settings are listed in [docs/features.md](docs/features.md).
Translations (English, Polish) are in `mixins/kidmode/lang/` and `runtime.rs`; the
Polish terminology is in [docs/translations/dictionary.md](docs/translations/dictionary.md).

`compatibility.json` pins the Horizon commit the mod is built on and records
the plan schema and the SHA-256 of every Horizon file it hooks. After changing
the hooks or the pin, refresh it with `python3 tools/update_compat.py`. CI
(`.github/workflows/ci.yml`) runs `cargo fmt --check`, clippy and the tests on
the Rust in `rust-toolchain.toml`, the one the launcher compiles mods with.

## Licence

Copyright © 2026 veinmine. Kid Mode is free software under the GNU General
Public License, version 3 or (at your option) any later version
(`GPL-3.0-or-later`); see [LICENSE](LICENSE).
