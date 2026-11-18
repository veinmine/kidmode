# What the hmod version can't do yet

Kid Mode 0.11.0 moved from the retired horizon-mixins bundle to the `.hmod` format and the mod API's
hooks, without mixins (veinmine, 2026-11-18: as few mixins as possible). The old version changed the mob
engine's own functions; in the new format those functions can't take mixins (they live in the
`horizon_client_lib` crate, below the game crate, and only functions marked
`#[horizon_mod_api_macros::replaceable]` there can). Everything else was ported to hooks. This page lists
what behaves differently from 0.10.0, and what still has to be checked in a game.

Checked against Horizon build `0b14f8a` (SDK of 18 Nov 2026). None of 0.11.0 has run in a game yet: the
first Nightly with mod tools hadn't shipped when it was written.

## Missing or different

| Feature | 0.10.0 | 0.11.0 | Why |
| --- | --- | --- | --- |
| Invulnerable mobs, undead in daylight | Invulnerable (and tamed) undead never caught fire in the sun, and didn't flee to the shade. | They catch fire, harmlessly, and still look for shade. | No hook on sun burning or on the flee-the-sun goal. |
| Invulnerable animals, fire panic | No panic or running from fire. | They no longer panic when hit, but may still run while burning. | No hook on the panic goal's fire check. |
| Friendly Elder Guardian | Its curse (mining fatigue) was dropped. | You still get the curse. | `mob_effect_applicable` doesn't say who applies the effect. |
| Friendly Ender Dragon | Treated you as a creative player: no charging or breath at you. | It still charges and breathes at you, but no hit, breath or wing push lands. | Its fight goes through game-crate code with no hook; a mixin could do it. |
| Friendly Wither, "fight other mobs" on | Never shot its idle volleys at random spots. | It may. | No hook on the Wither's ranged attack. |
| Leads | The lead hung from the middle of an enemy's body. | It hangs where vanilla puts it. | No hook on the lead's position. |
| Taming hints, appearing | The bubble faded in and out. | It grows in and shrinks away. | A mob's extras are drawn opaque. |
| Taming hints, wolves and ocelots | An angry wolf showed no bubble; an ocelot that trusts you showed the heart. | A wolf always shows the bone; an ocelot shows the fish until tamed. | A wolf's anger and an ocelot's trust can't be read on the client. |
| Upgrading | Mobs tamed with 0.10.0 kept the `kidmode_tamed` tag. | The tag still tames a mob. No other compatibility (veinmine: none needed before 1.0). | — |

## To check in a game

These are written against the API's documentation, not tested yet:

- Friendly mobs: `living_change_target` really stops every target pick, brains included (warden,
  piglins, hoglins, goats). In 0.9 the old native hook fired after the mob had acted; the API's
  hook is documented as firing before.
- A friendly mob's hit, push and explosion on a player is refused through the mob-type cache the
  server fills after each mob's tick (`logic::note_mob`).
- An invulnerable creeper: its explosion is cancelled. Check that it isn't removed anyway, and that a
  flint-and-steel ignition stops (the mod clears its `ignited` flag).
- A witch's fire resistance is refused at `living_entity_use_item_start`; witches may drink another way.
- Auto sleep: `set_clock_time` to the next morning, weather cleared with `set_weather(0, 0, …)`,
  time since rest reset for every Overworld player.
- Leads: `entity_interact` ties the lead with `leash_to` before vanilla refuses it on a monster.
- Taming hints: the white sprite `kidmode:block/white` loads into the block atlas, the quads face the
  camera (not culled from behind), the head shows the tamer's skin, and items are flattened.
- Settings: the page shows the eleven toggles, and a dedicated server reads its own setting files
  (`Host::setting` on the server).
- The package: `cargo horizon-mod build` packs the `Cargo.lock` Cargo generates (it isn't committed;
  the mod has no registry dependencies of its own). Check that the launcher's build accepts it.
