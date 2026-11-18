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
| [Tame mobs for loot](#tame-mobs-for-loot) | Tame mobs for loot (`tame_for_loot`) | ON |
| [Taming hints](#taming-hints) | Taming hints (`taming_hints`) | ON |

## Invulnerable player

The player never takes damage: mobs, falls, fire, lava, drowning, explosions,
suffocation, and starving on an empty food bar. A hit is refused before it
lands, so there is no knockback or hurt flash either. Hunger still drains as
usual. `/kill` and falling out of the world still work.

## Mob groups

Vanilla mobs only; mobs from other mods are not affected yet. The lists are
`HOSTILE`, `NEUTRAL` and `ANIMALS` in `src/logic.rs`.

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

How it works: whenever a friendly mob picks you as its target (a goal, a
brain, or revenge after you hit it), the pick is dropped, so it never chases
or attacks you, and a target it still holds after its turn is cleared.
Creepers never fuse because they never target you. As a backup, any hit or
push on you from a friendly mob, its projectile or its explosion is refused
(a sting, a slime's contact damage, a goat's ram, the Warden's sonic boom,
the Wither's skulls, the Ender Dragon's breath and wings), and an enderman
doesn't mind being stared at.

Bosses: the Warden never targets you, and its sonic boom can't hurt you. A
friendly Wither's spawn blast and skulls don't explode (unless "Friendly mobs
fight other mobs" is on) and never hurt you. The Ender Dragon's breath, hits
and wing pushes never land on you. What still differs from 0.10 (the Elder
Guardian's curse, the dragon's charges) is in [missing.md](missing.md).

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
piglin, iron golem). They never take damage and don't react to fire, and witches don't drink fire
resistance.

- Undead mobs still catch fire in daylight, harmlessly (see
  [missing.md](missing.md)).

- An invulnerable creeper never explodes, not even when lit with flint and
  steel.
- The Ender Dragon has its own setting, below.

## Invulnerable Ender Dragon

Off by default. The Ender Dragon refuses every hit (arrows, melee, beds,
explosions, crystals blowing up next to it) before it reacts, so it takes no
damage and doesn't change its flight because of it. `/kill` still works. With
it on, the dragon fight can't be won.

## Always sunny

Off by default. Rain and thunderstorms are refused the moment they would
start, so the sky stays clear.

## Auto sleep

Off by default. When the day ends (the time a bed would let you sleep, 12542
on the day clock) the game skips to the next morning, as when everyone
sleeps: the clock jumps, rain clears if the weather cycle runs, and the night
counts as rest for every player in the Overworld, so phantoms don't come.
Only in the Overworld, and only when the `advance_time` game rule is on.

## Tame mobs for loot

On by default, and nothing changes until a player feeds a mob. Since mobs here
can't be killed for their drops, a mob fed its taming item may be tamed (1 in
3, like a wolf: hearts on success, smoke on a miss). The item is used up, except
in creative; a milk bucket leaves an empty bucket.

A tamed mob drops one item of its loot every 5 to 10 minutes, like a chicken's
egg (adults only). It is always friendly, never despawns, never burns in
daylight and stays where it is. It shows a few green sparkles now and then
and a heart in its thought bubble (see Taming hints). The tamed state is saved
with the mob as the attachment `kidmode:tamed`; `/tag <mob> add kidmode_tamed`
tames one too. Any player can tame a mob and collect its loot.

Every enemy and neutral mob in the table can be put on a lead, tamed or not,
to bring it home. Animals keep vanilla's leads (none on fish, squid or bats).
A tamed enderman or shulker never teleports, so it doesn't break its lead.

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
| Cow, mooshroom | leather, raw beef | golden carrot |
| Chicken | feather, raw chicken (eggs as usual) | golden carrot |
| Sheep | raw mutton (shear for wool as usual) | golden carrot |
| Pig | raw porkchop | golden carrot |
| Goat | nothing | golden carrot |
| Rabbit | rabbit hide, raw rabbit | apple |
| Polar bear | raw cod, raw salmon | raw cod or salmon |
| Panda | bamboo | sugar cane |
| Fox | sweet berries | raw chicken |
| Bee | honeycomb | sugar |
| Frog | froglight of its colour (ochre, verdant, pearlescent) | spider eye |
| Armadillo | nothing (it sheds scutes as usual) | apple |
| Sniffer | nothing (it digs up seeds as usual) | moss block |
| Camel | nothing | hay bale |
| Strider | string | crimson fungus |
| Bat | nothing | melon slice |
| Squid | ink sac | kelp |
| Glow squid | glow ink sac | kelp |
| Dolphin | raw cod | kelp |
| Cod, salmon, tropical fish, pufferfish | the fish itself, bone meal | kelp |
| Turtle | seagrass, turtle scute | kelp |
| Axolotl | nothing | kelp |

Vanilla's pets keep their vanilla taming and, once tamed, drop loot the same
way (adults, every 5 to 10 minutes, with sparkles): cat (string), parrot
(feather), horse, donkey, mule, llama and trader llama (leather), skeleton
horse (bone), zombie horse (rotten flesh). Wolves, ocelots and nautili drop
nothing. Left out: the tadpole (it grows into a frog) and the happy ghast.

With the setting off, feeding does nothing, tamed mobs stop dropping loot and
count as untamed, and leads work as in vanilla; they stay tamed for when it
is turned on again. The tables are `TAMEABLE` and `PET_LOOT` in `src/logic.rs`;
the design is in [taming.md](taming.md).

## Taming hints

On by default. Look at a mob that can be tamed, from up to 8 blocks away with
no block in between, and a thought bubble shows over its head: the item that
tames it, or a heart once it is tamed. A Kid Mode mob someone else tamed shows
that player's head instead (with their skin), so you can tell whose it is. A
mob tamed with `/tag` has no known tamer and shows the heart to everyone. Vanilla's tamed mobs always show the heart (their owner isn't
sent to players' games). Only the mob under the crosshair (the
nearest one along the view) shows one.

- **Kid Mode's tameable mobs:** the table above, while Tame mobs for loot
  is on.
- **Vanilla's tameable mobs:** wolf (bone), cat and ocelot (raw cod or
  salmon), parrot (seeds: wheat, melon, pumpkin, beetroot, torchflower, or a
  pitcher pod), nautilus (pufferfish, adults only), and horse, donkey, mule,
  llama and trader llama (a saddle: they are tamed by riding them). A baby
  nautilus and a baby horse or llama show no bubble.
- A mob with several taming items (raw meat, flowers, fish, seeds) shows each
  in turn, one a second.
- The bubble grows in from its tail (0.15 s), stays for 1.5 s after you look
  away (or until you look back), then shrinks away (0.4 s). Looking back while
  it shrinks brings it back from where it was.

The bubble is drawn on your own screen only (after the mob, through the API's
`render_living_post`), turned to face the camera and full bright, so it reads
at night too. The item is flattened onto the bubble, so it stays inside it
from any angle. The bubble sits above a mob's name when it has one.

Every player sees this, on LAN and on servers too: the server sends each
player's game a synced attachment, `kidmode:tamed`, on every mob the mod
tamed, with its tamer's id, name and skin profile. Knowing the tamer changes
nothing else: anyone can still collect a tamed mob's loot and lead it.
Vanilla's tamed state is sent the usual way. The lists are `VANILLA_TAMEABLE`
and `TAMEABLE` in `src/logic.rs`, the drawing is `src/hint.rs`.
