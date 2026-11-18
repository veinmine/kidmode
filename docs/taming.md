# Design: Tame mobs for loot

Status: agreed with veinmine on 2026-11-17, implemented in 0.7.0. Every animal
added on 2026-11-17 (0.10.0, list approved by veinmine).

With Kid Mode, mobs are friendly and can be invulnerable, so killing them for
their drops is out. This feature gives another way to get that loot: feed a
mob a fitting item to tame it, and the tamed mob then drops loot now and then,
the way a chicken lays eggs.

## Setting

| Setting | File | Default |
|---|---|---|
| Tame mobs for loot (Polish: **Oswajanie stworzeń**; until 0.9.2 "Tame enemies for loot", "Oswajanie wrogów") | `tame_for_loot` | ON |

ON by default because nothing changes until a player feeds a mob. With it OFF,
feeding does nothing, already tamed mobs stop dropping loot and lose their
sparkles, and leads work only on the mobs vanilla allows. They keep their
tamed tag, so they come back if the setting is turned on again.

## Mobs, drops and taming items

Each taming item can be got without the mob's own drop, so taming is never a
chicken-and-egg problem. One item can tame several mobs.

| Mob | Loot it drops | Tame with |
|---|---|---|
| Zombie, husk, zombie villager | rotten flesh | any raw meat (beef, porkchop, chicken, mutton, rabbit) |
| Zombified piglin | rotten flesh, gold nugget | any raw meat |
| Drowned | rotten flesh, copper ingot | raw cod or raw salmon |
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
| Chicken | feather, raw chicken | golden carrot |
| Sheep | raw mutton | golden carrot |
| Pig | raw porkchop | golden carrot |
| Goat | nothing | golden carrot |
| Rabbit | rabbit hide, raw rabbit | apple |
| Polar bear | raw cod, raw salmon | raw cod or raw salmon |
| Panda | bamboo | sugar cane |
| Fox | sweet berries | raw chicken |
| Bee | honeycomb | sugar |
| Frog | froglight of its colour | spider eye |
| Armadillo | nothing | apple |
| Sniffer | nothing | moss block |
| Camel | nothing | hay bale |
| Strider | string | crimson fungus |
| Bat | nothing | melon slice |
| Squid | ink sac | kelp |
| Glow squid | glow ink sac | kelp |
| Dolphin | raw cod | kelp |
| Cod, salmon, tropical fish, pufferfish | the fish itself, bone meal | kelp |
| Turtle | seagrass, turtle scute | kelp |
| Axolotl | nothing | kelp |

Cow and chicken came first (0.7.0) because with Invulnerable animals on (the
default), leather and feathers can't be got any other way, and without them
there are no books (so no enchanting) and no arrows. In 0.10.0 veinmine asked
for every remaining animal to be tameable, even with no loot, and for the
farm animals to drop their meat too.

Animal taming items are never that animal's breeding food and don't clash
with vanilla's uses on it (milking, shearing, bowls, buckets, dolphins' fish).
A frog drops the froglight it would make by eating a magma cube: ochre
(temperate), verdant (cold), pearlescent (warm). An armadillo and a sniffer
drop nothing extra because they already shed scutes and dig up seeds.

### Vanilla's pets

Pets that vanilla already tames keep their vanilla taming (bone, fish, seeds,
riding) and are not in the table above. Once tamed the vanilla way (their own
tame flag) they drop loot like a Kid Mode mob, with sparkles:

| Pet | Loot |
|---|---|
| Cat | string |
| Parrot | feather |
| Horse, donkey, mule, llama, trader llama | leather |
| Skeleton horse | bone |
| Zombie horse | rotten flesh |

Wolves, ocelots and nautili drop nothing. Left out entirely: the tadpole (it
grows into a frog, which can be tamed), the happy ghast (already a pet raised
from a dried ghast), villagers, wandering traders and golems.

Left out: the illagers, the bosses, and rare drops (wither skull, evoker's
totem, a zombie's iron ingot, carrot and potato).

The table lives in code in `src/logic.rs` (one list of mob, loot,
taming items), so it's easy to change.

## Taming

- Right-click the mob holding its taming item. Like taming a wolf, each item
  has a 1 in 3 chance: smoke particles on a miss, hearts on success. The item
  is used up, except in creative.
- A baby mob (baby zombie, baby hoglin and so on) can be tamed the same way.
- Any player can tame a mob and any player can collect its loot or lead it.
  The mod remembers who tamed it (veinmine, 2026-11-17: only to
  show their head in the taming-hint bubble to other players; it limits
  nothing).
- Taming an already tamed mob does nothing and doesn't use up the item.

## A tamed mob

- Always friendly to players, even with Friendly enemies or Friendly neutral
  mobs OFF.
- Never despawns (it becomes persistent, like a named mob).
- Never burns in daylight.
- Stays where it is, as before: it doesn't follow players and can't be
  ordered to sit. A lead moves it (see below).
- Doesn't change how it takes damage. It's invulnerable only if Invulnerable
  enemies or Invulnerable animals covers it.

## Loot

Like a chicken's egg: every 5 to 10 minutes (6000 to 12000 ticks, random each
time) a tamed adult drops one item, picked at random from its loot list, with
the item-pop sound. Babies drop nothing until they grow up. The timer runs only
while the mob's chunk is loaded, as with eggs.

## Leads

Every enemy and neutral mob in the table above can be put on a lead, **tamed
or not**, so that a mob can be led home and tamed there, or a tamed one moved.
Vanilla doesn't allow leads on monsters. Animals keep vanilla's lead rules
(so no leads on fish, squid or bats). Leads work as usual otherwise: shears cut them, and
a lead breaks if pulled too far.

Since 0.11.0 this doesn't work: the mod API can't make a monster leashable
yet (`EntityRef::leash_to` is refused for a mob vanilla can't leash), so leads
work as in vanilla until it can ([missing.md](missing.md)).

## How a tamed mob is shown

- **Hearts** when it's tamed, then **a few green sparkles** (the
  happy-villager particle) above its head now and then, about every 10 to 20
  seconds.
- **A heart in its thought bubble** when you look at it (Taming hints, see
  features.md). Up to 0.8.0 a tamed mob got a name ("Tame zombie"); veinmine
  dropped that for the heart on 2026-11-17. Mobs tamed before keep the name
  they got, as if from a name tag.

The sparkles work on LAN and on servers, because particles are sent to every
player. Green eyes were considered and left for later: there's no eye texture
for zombies, skeletons or creepers, and a mixin mod can't ship textures.

## How it works (for implementation)

On the mod API's hooks (`src/lib.rs`), since 0.11.0:

- **Feeding and leads:** `entity_interact` on the server, before the mob's own
  interactions (name tags, buckets); the player is the host's
  `acting_player`. A lead on a leashable mob that isn't on one is tied with
  `EntityRef::leash_to`, which the game refuses for monsters for now.
- **Tamed state:** the attachment `kidmode:tamed` (`src/tamed.rs`): saved
  with the mob, copied on conversion, and synced to every player tracking it,
  with the tamer's id, name and profile. The entity tag `kidmode_tamed`
  (`/tag @e[type=zombie,limit=1,sort=nearest] add kidmode_tamed`) counts too;
  such a mob gets the attachment (without a tamer) on its next tick.
- **Loot, sparkles, persistence:** `entity_tick_post` (the mod listens through
  `entity_tick_listened`) copies the tamed mobs into `logic`'s `TAMED` map with
  their timers, marks them persistent, and drops loot with
  `EntityRef::spawn_at_location`. A frog's froglight comes from its saved
  `variant`.
- **Vanilla's pets:** counted as tamed when `LivingRef::is_tamed` says so (no
  attachment: vanilla sends its tame flag itself).
- **Hearts and smoke:** sent at once with `LevelRef::send_particles`.
- **No despawning, no teleports:** `mob_despawn` (deny) and
  `ender_entity_teleport` (cancel) for tamed mobs.
- **Friendliness:** `logic::friendly` counts tamed mobs.
- Not possible through the API yet: the lead hanging from the middle of the
  body, and tamed undead not burning in daylight ([missing.md](missing.md)).
