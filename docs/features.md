# Kid Mode features

Kid Mode (mod id `mord`) makes the world gentler. Every feature, with its
setting. Settings live under **Options > Mods > Kid Mode** and are kept in `horizon-mods/.horizon-mod-data/mord/`
in the game folder, one file per setting holding `true` or `false`. On a
dedicated server the server's own copies of these files decide.

| Feature | Setting (file) | Default |
|---|---|---|
| [Invulnerable player](#invulnerable-player) | Invulnerable player (`invulnerable_player`) | ON |
| [Friendly enemies](#friendly-enemies) | Friendly enemies (`friendly_enemies`) | ON |
| [Friendly neutral mobs](#friendly-neutral-mobs) | Friendly neutral mobs (`friendly_neutral_mobs`) | ON |
| [Friendly mobs fight other mobs](#friendly-mobs-fight-other-mobs) | Friendly mobs fight other mobs (`friendly_mobs_fight_mobs`) | OFF |
| [Invulnerable animals](#invulnerable-animals) | Invulnerable animals (`invulnerable_animals`) | ON |
| [Invulnerable enemies](#invulnerable-enemies) | Invulnerable enemies (`invulnerable_enemies`) | OFF |
| [Invulnerable Ender Dragon](#invulnerable-ender-dragon) | Invulnerable Ender Dragon (`invulnerable_ender_dragon`) | OFF |
| [Always sunny](#always-sunny) | Always sunny (`always_sunny`) | OFF |
| [Auto sleep](#auto-sleep) | Auto sleep (`auto_sleep`) | OFF |

## Invulnerable player

The player never takes damage: mobs, falls, fire, lava, drowning, explosions,
suffocation, and starving on an empty food bar. A hit is refused before it
lands, so there is no knockback or hurt flash either. Hunger still drains as
usual. `/kill` and falling out of the world still work.

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
skips its spawn blast and shoots no skulls at all (with "Friendly mobs fight
other mobs" on it may shoot at mobs, but never its idle volleys at random
spots). The Ender Dragon treats
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
- The Ender Dragon has its own setting, below.

## Invulnerable Ender Dragon

Off by default. The Ender Dragon refuses every hit (arrows, melee, beds,
explosions, crystals blowing up next to it) before it reacts, so it takes no
damage and doesn't change its flight because of it. `/kill` still works. With
it on, the dragon fight can't be won.

## Always sunny

Off by default. Rain and thunderstorms are cleared the moment they start, so
the sky stays clear (the weather cycle keeps running underneath).

## Auto sleep

Off by default. When the day ends (the time a bed would let you sleep, 12542
on the day clock) the game skips to the next morning, exactly as when everyone
sleeps: the clock jumps, rain clears if the weather cycle runs, and the night
counts as rest, so phantoms don't come. Only in the Overworld, and only when
the `advance_time` game rule is on.
