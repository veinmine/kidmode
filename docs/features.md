# Mord features

Every feature of the mod, with its setting. Settings live under
**Options > Mods > Mord** and are kept in `horizon-mods/.horizon-mod-data/mord/`
in the game folder (one file per setting).

| Feature | Setting | Default |
|---|---|---|
| [Friendly enemies](#friendly-enemies) | Friendly enemies | ON |

## Friendly enemies

Hostile mobs become passive towards players: they never target or attack you,
even after you hit them first. You can still attack and kill them.

- Covers vanilla's hostile mobs, including those that only turn hostile when
  provoked (enderman, piglins, zombified piglins, spiders). Wolves, iron
  golems, bees, llamas and other non-hostile mobs keep their normal behaviour.
- Mobs from other mods are not affected yet.
- Turning it on mid-fight: a mob already chasing you may keep following, but
  its hits are cancelled.
- Setting file: `friendly_enemies` (`true` / `false`). On a dedicated server
  the server's own copy of this file decides.
- Implementation: `mixins/mord/target.rs` (target changes) and `damage.rs`
  (hits), logic in `runtime.rs`.
