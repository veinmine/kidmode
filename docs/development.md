# Building Kid Mode

Kid Mode is a native [Horizon](https://playhorizon.dev) mod, packaged as one `.hmod` (Horizon's mod format since
the mod API, November 2026). It uses the mod API's hooks only, no mixins.

```sh
cargo horizon-mod check                                          # fetches the SDK into .horizon/sdk/ (once)
cargo test                                                       # the mod's own logic
horizon-launcher native-build . --trust-native-code              # build it into the selected instance
cargo horizon-mod build                                          # the package: target/horizon-mod/kidmode.hmod
```

`cargo horizon-mod check` also checks the mod for WebAssembly, which needs `rustup target add
wasm32-unknown-unknown`; once the SDK is in `.horizon/sdk/`, plain `cargo check`, `cargo clippy` and
`cargo test` work without it.

The layout is the native `.hmod` one, at the repository root:

- `horizon-mod.toml`: the manifest; `[native]` pins the exact Horizon build (`horizon`) and horizon-mixins
  commit (`mixins`) it is made for. With no `[native.*]` tables, the package lists the API items it uses
  (`uses.json`), so the launcher can build it unchanged on a newer Horizon build where those items didn't
  change (`cargo horizon-mod fits`). To move the pin, run `cargo horizon-mod update`.
- `src/lib.rs`: the hooks (`impl Hooks`); `src/logic.rs`: what the mod decides, free of the game, with the
  tests; `src/hint.rs`: the taming hints; `src/tamed.rs`: the `kidmode:tamed` attachment.
- `assets/kidmode/`: translations (English, Polish) and the white sprite the taming bubble is drawn with.

Features and their settings are listed in [features.md](features.md); what the API can't do yet,
compared with the old mixin version, in [missing.md](missing.md). The Polish terminology is in
[translations/dictionary.md](translations/dictionary.md). CI (`.github/workflows/ci.yml`) runs
`cargo fmt --check`, clippy and the tests against the pinned SDK.
