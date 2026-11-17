# Kid Mode features

Kid Mode (mod id `kidmode`) makes the world gentler. Every feature, with its
setting. Settings live under **Options > Mods > Kid Mode** and are kept in `horizon-mods/.horizon-mod-data/kidmode/`
in the game folder, one file per setting holding `true` or `false`. On a
dedicated server the server's own copies of these files decide.

The mod's name, description and settings follow the game's language: English
and Polish (**Tryb dla dzieci**). The Polish terms are listed in
[translations/dictionary.md](translations/dictionary.md).

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
| [Tame enemies for loot](#tame-enemies-for-loot) | Tame enemies for loot (`tame_for_loot`) | ON |
| [Taming hints](#taming-hints) | Taming hints (`taming_hints`) | ON |

## Invulnerable player

The player never takes damage: mobs, falls, fire, lava, drowning, explosions,
suffocation, and starving on an empty food bar. A hit is refused before it
lands, so there is no knockback or hurt flash either. Hunger still drains as
usual. `/kill` and falling out of the world still work.

## Mob groups

Vanilla mobs only; mobs from other mods are not affected yet. The lists are
`HOSTILE`, `NEUTRAL` and `ANIMALS` in `mixins/kidmode/runtime.rs`.

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
- Undead animals (the zombie horse) don't catch fire in daylight.
- Explosions don't push them. Players, other mobs and flowing water still can.
- `/kill` and falling out of the world still remove them.

## Invulnerable enemies

Off by default. The same as Invulnerable animals, for enemies: all hostile
mobs plus the neutral mobs that are not animals (enderman, piglin, zombified
piglin, iron golem). They never take damage and don't react to fire, and witches don't drink fire
resistance.

- Undead mobs (zombies, drowned, skeletons, strays, bogged, wither skeletons,
  phantoms, zombie villagers, zombie nautiluses) don't catch fire in daylight
  at all, and a helmet they wear doesn't wear down in the sun. Lava, fire
  blocks, flaming arrows and the like can still set them alight, harmlessly.

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

## Tame enemies for loot

On by default, and nothing changes until a player feeds a mob. Since mobs here
can't be killed for their drops, a mob fed its taming item may be tamed (1 in
3, like a wolf: hearts on success, smoke on a miss). The item is used up, except
in creative; a milk bucket leaves an empty bucket.

A tamed mob drops one item of its loot every 5 to 10 minutes, like a chicken's
egg (adults only). It is always friendly, never despawns, never burns in
daylight and stays where it is. It shows a few green sparkles now and then
and a heart in its thought bubble (see Taming hints), and carries the entity tag
`kidmode_tamed` (`/tag <mob> add kidmode_tamed` tames one too). Any player can tame
a mob and collect its loot.

Every mob in the table can be put on a lead, tamed or not, to bring it home.
The lead hangs from the centre of its body rather than its head (cows and
chickens keep vanilla's). A tamed enderman or shulker never teleports, so it
doesn't break its lead.

| Mob | Loot | Tame with |
|---|---|---|
| Zombie, husk, zombie villager | rotten flesh | any raw meat |
| Zombified piglin | rotten flesh, gold nugget | any raw meat |
| Drowned | rotten flesh, copper ingot | raw cod or salmon |
| Skeleton, stray, bogged | bone, arrow | milk bucket |
| Wither skeleton | coal, bone | milk bucket |
| Creeper | gunpowder | any flower |
| Spider, cave spider | string, spider eye | sweet berries |
| Enderman | ender pearl | grass block |
| Witch | redstone, glowstone dust, sugar, stick, glass bottle | red or brown mushroom |
| Slime | slimeball | sugar |
| Magma cube | magma cream | coal or charcoal |
| Blaze | blaze rod | coal or charcoal |
| Guardian | prismarine shard, prismarine crystals | tropical fish |
| Shulker | shulker shell | chorus fruit |
| Breeze | breeze rod | feather |
| Hoglin | leather, raw porkchop | crimson fungus |
| Ghast | ghast tear, gunpowder | snowball |
| Phantom | phantom membrane | glow berries |
| Cow | leather | golden carrot |
| Chicken | feather | golden carrot |

With the setting off, feeding does nothing, tamed mobs stop dropping loot and
count as untamed, and leads work as in vanilla; they keep their tag.
The table is `TAMEABLE` in `mixins/kidmode/runtime.rs`; the design is in
[taming.md](taming.md).

## Taming hints

On by default. Look at a mob that can be tamed, from up to 8 blocks away with
no block in between, and a thought bubble shows over its head: the item that
tames it, or a heart once it is tamed. A Kid Mode mob someone else tamed shows
that player's head instead (with their skin), so you can tell whose it is. A
mob tamed with `/tag`, or before 0.9.1, has no known tamer and shows the heart
to everyone. Vanilla's tamed mobs always show the heart (their owner isn't
sent to players' games). Only the mob under the crosshair (the
nearest one along the view) shows one.

- **Kid Mode's tameable mobs:** the table above, while Tame enemies for loot
  is on.
- **Vanilla's tameable mobs:** wolf (bone), cat and ocelot (raw cod or
  salmon), parrot (seeds: wheat, melon, pumpkin, beetroot, torchflower, or a
  pitcher pod), nautilus (pufferfish, adults only), and horse, donkey, mule,
  llama and trader llama (a saddle: they are tamed by riding them). An angry
  wolf, a baby nautilus and a baby horse or llama show no bubble; an ocelot
  that trusts you shows the heart.
- A mob with several taming items (raw meat, flowers, fish, seeds) shows each
  in turn, one a second.

The bubble is drawn on your own screen only (a layer on the mob through
Horizon's `native_mob_layer`), turned to face the camera and full bright, so
it reads at night too. The item is flattened onto the bubble, so it stays
inside it from any angle, and drawn without the world's shading, in its
inventory colours. The bubble sits above a mob's name when it has one.

Every player sees this, on LAN and on servers too: the server sends each
player's game a synced attachment, `kidmode:tamed`, on every mob the mod
tamed, with its tamer's id, name and skin profile (the `kidmode_tamed` tag
itself isn't sent to players). Knowing the tamer changes nothing else: anyone
can still collect a tamed mob's loot and lead it. Vanilla's tamed
state is sent the usual way. The lists are `VANILLA_TAMEABLE` and `TAMEABLE` in
`mixins/kidmode/runtime.rs`, the drawing is `mixins/kidmode/tame_hint.rs`.
