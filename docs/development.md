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

## Watching Horizon

`.github/workflows/horizon-watch.yml` runs every Monday (and on demand from the Actions tab) against the
current Horizon Nightly, the build named by its SDK release's `sdk.json`. It uses Horizon's own
`cargo horizon-mod`, built at that Nightly, and keeps at most one open issue per label:

- **`horizon-compat`**: `cargo horizon-mod fits` (does the released mod fit the Nightly unchanged), then a
  repin to the Nightly in the runner (`cargo horizon-mod update`, nothing committed) with clippy and the
  tests. Opened when any of it fails, closed when it passes again.
- **`horizon-api`**: nearly every Nightly changes the mod API, so this one is only for changes that matter:
  an item Kid Mode uses changed (the report `cargo horizon-mod update` prints), or an item matching a rule
  in [`.github/horizon-watchlist.txt`](../.github/horizon-watchlist.txt) was added, changed or removed in
  `cargo horizon-mod sdk diff` from the pin. The rules are mostly the gaps in [missing.md](missing.md),
  so a new hook that would bring a feature back shows up; drop a rule once its feature is back. The full
  diff is always in the run's `horizon-watch-report` artifact.

An open issue is edited in place and gets a comment only when what it reports changes.
Horizon isn't a crates.io dependency (the API crate is a path into `.horizon/sdk/`), so Dependabot and
Renovate can't follow it.
