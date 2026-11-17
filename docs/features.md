# Mord features

Every feature of the mod, with its setting. Settings live under
**Options > Mods > Mord** and are kept in `horizon-mods/.horizon-mod-data/mord/`
in the game folder, one file per setting holding `true` or `false`. On a
dedicated server the server's own copies of these files decide.

| Feature | Setting (file) | Default |
|---|---|---|
| [Friendly enemies](#friendly-enemies) | Friendly enemies (`friendly_enemies`) | ON |
| [Friendly neutral mobs](#friendly-neutral-mobs) | Friendly neutral mobs (`friendly_neutral_mobs`) | ON |
| [Friendly mobs fight other mobs](#friendly-mobs-fight-other-mobs) | Friendly mobs fight other mobs (`friendly_mobs_fight_mobs`) | OFF |
| [Invulnerable animals](#invulnerable-animals) | Invulnerable animals (`invulnerable_animals`) | ON |
| [Invulnerable enemies](#invulnerable-enemies) | Invulnerable enemies (`invulnerable_enemies`) | OFF |

## Mob groups

Vanilla mobs only; mobs from other mods are not affected yet. The lists are
`HOSTILE`, `NEUTRAL` and `ANIMALS` in `mixins/mord/runtime.rs`.

- **Enemies (hostile):** zombies, skeletons, creepers, spiders, witches,
  illagers, slimes, phantoms, guardians and the rest of vanilla's hostile
  mobs, including the bosses: the Warden, the Elder Guardian, the Wither and
  the Ender Dragon.
- **Neutral:** bee, dolphin, enderman, goat, iron golem, llama, panda, piglin,
  polar bear, trader llama, wolf, zombified piglin.
- **Animals:** vanilla's passive and neutral creatures, on land and in water
  (cows, pigs, sheep, horses, fish, wolves, bees and so on). Not villagers or
  iron golems.

## Friendly enemies

Hostile mobs are passive towards players. They never target you, chase you or
attack you, and they don't strike back when you hit them. You can still attack
and kill them.

How it works: the mob engine's "can this mob attack a player?" check
(vanilla's `canAttack`) says no for them, so they never pick you as a target,
take no revenge, and drop a target they already had. Creepers never fuse
because they never target you. As a backup, any hit, sting, harmful effect,
contact damage (slimes), sonic boom or curse that a friendly mob's tick aims
at you is dropped.

To a friendly mob you count as a creative player in every targeting check,
and the mob-specific ways of getting angry are switched off too: an
enderman's stare, a piglin's anger at a looted chest or mined gold, a goat's
ram, a bee's or wolf's anger.

Bosses: the Warden never gets angry at you, so it never roars, chases or
sonic-booms you. The Elder Guardian's curse is dropped. A friendly Wither
skips its spawn blast and its heads never pick you. The Ender Dragon treats
you like a creative player: no charging, no breath, no wing hits or pushes.

## Friendly neutral mobs

The same as Friendly enemies, for neutral mobs: hitting a wolf or a zombified
piglin, looking at an enderman or taking honey makes none of them attack you.

## Friendly mobs fight other mobs

Off by default: friendly mobs (the hostile and neutral mobs made friendly by
the two settings above) attack no other mob either. Iron golems leave zombies
alone, wolves ignore skeletons, and zombies ignore villagers. Turn it on to let
friendly mobs fight each other and other mobs as usual. Mobs that are not
friendly keep fighting whatever they normally fight.

## Invulnerable animals

Animals never take damage: attacks, fire, lava, falling, drowning, explosions.
The hit is refused at its very start, as with vanilla's `Invulnerable` tag, so
there is no hurt flash, no knockback and no attacker to remember. They don't
react in any way: no panic, no running from fire, no revenge.

- They can still be on fire; it just doesn't hurt them.
- Explosions don't push them. Players, other mobs and flowing water still can.
- `/kill` and falling out of the world still remove them.

## Invulnerable enemies

Off by default. The same as Invulnerable animals, for enemies: all hostile
mobs plus the neutral mobs that are not animals (enderman, piglin, zombified
piglin, iron golem). They never take damage and don't react to fire. Zombies and skeletons burn in daylight
without seeking shade, and witches don't drink fire resistance.

- An invulnerable creeper never explodes, not even when lit with flint and
  steel.
- The Ender Dragon is not covered (it is not a mob of the engine): it can still
  be killed.
