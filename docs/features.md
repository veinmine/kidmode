# Mord features

Every feature of the mod, with its setting. Settings live under
**Options > Mods > Mord** and are kept in `horizon-mods/.horizon-mod-data/mord/`
in the game folder, one file per setting holding `true` or `false`. On a
dedicated server the server's own copies of these files decide.

| Feature | Setting (file) | Default |
|---|---|---|
| [Friendly enemies](#friendly-enemies) | Friendly enemies (`friendly_enemies`) | ON |
| [Friendly neutral mobs](#friendly-neutral-mobs) | Friendly neutral mobs (`friendly_neutral_mobs`) | ON |
| [Invulnerable animals](#invulnerable-animals) | Invulnerable animals (`invulnerable_animals`) | ON |
| [Invulnerable enemies](#invulnerable-enemies) | Invulnerable enemies (`invulnerable_enemies`) | OFF |

Mob groups are vanilla mobs only; mobs from other mods are not affected yet.
The lists are `HOSTILE`, `NEUTRAL` and `ANIMALS` in `mixins/mord/runtime.rs`.

- **Enemies (hostile):** zombies, skeletons, creepers, spiders, witches,
  illagers, slimes, phantoms, guardians, the Warden, the Wither, the Ender
  Dragon and the rest of vanilla's hostile mobs.
- **Neutral:** bee, dolphin, enderman, goat, iron golem, llama, panda, piglin,
  polar bear, trader llama, wolf, zombified piglin.
- **Animals:** vanilla's passive and neutral creatures, on land and in water
  (cows, pigs, sheep, horses, fish, wolves, bees and so on). Not villagers or
  iron golems.

## Friendly enemies

Hostile mobs become passive towards players: they never target or attack you,
even after you hit them first. You can still attack and kill them.

- A mob already chasing you when you turn it on may keep following, but its
  hits are cancelled.

## Friendly neutral mobs

The same as Friendly enemies, for neutral mobs: angering an enderman, hitting a
wolf or a zombified piglin, or taking from a bee's nest makes none of them
attack you.

## Invulnerable animals

Animals take no damage from anything: attacks, fire, lava, falling, drowning,
explosions. A hit on them is cancelled entirely, so they show no hurt flash,
are not knocked back, don't panic or strike back, and drop nothing.

- They can still catch fire; the fire just doesn't hurt them.
- Known limit: vanilla's panic behaviour also makes an animal run while it is
  on fire (not only when hurt), and no hook point turns that off, so a burning
  animal still runs until the fire goes out.
- `/kill` and falling out of the world still remove them.

## Invulnerable enemies

The same as Invulnerable animals, for hostile mobs: they never take damage, so
zombies and skeletons survive daylight and can't be killed by hits, lava or
explosions. Off by default.
