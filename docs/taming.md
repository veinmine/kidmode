# Design: Tame enemies for loot

Status: agreed with veinmine on 2026-11-17, implemented in 0.7.0.

With Kid Mode, mobs are friendly and can be invulnerable, so killing them for
their drops is out. This feature gives another way to get that loot: feed a
mob a fitting item to tame it, and the tamed mob then drops loot now and then,
the way a chicken lays eggs.

## Setting

| Setting | File | Default |
|---|---|---|
| Tame enemies for loot (Polish: **Oswajanie wrogów**) | `tame_for_loot` | ON |

ON by default because nothing changes until a player feeds a mob. With it OFF,
feeding does nothing, already tamed mobs stop dropping loot and lose their
sparkles, and leads work only on the mobs vanilla allows. They keep their
names and their tamed tag, so they come back if the setting is turned on again.

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
| Cow | leather | golden carrot |
| Chicken | feather | golden carrot |

Cow and chicken are here because with Invulnerable animals on (the default),
leather and feathers can't be got any other way, and without them there are
no books (so no enchanting) and no arrows. They drop no meat. The golden
carrot doesn't clash with breeding (wheat and seeds stay breeding items).

Left out: the illagers, the bosses, and rare drops (wither skull, evoker's
totem, a zombie's iron ingot, carrot and potato).

The table lives in code in `mixins/mord/runtime.rs` (one list of mob, loot,
taming items), so it's easy to change.

## Taming

- Right-click the mob holding its taming item. Like taming a wolf, each item
  has a 1 in 3 chance: smoke particles on a miss, hearts on success. The item
  is used up, except in creative.
- A baby mob (baby zombie, baby hoglin and so on) can be tamed the same way.
- Any player can tame a mob and any player can collect its loot. A tamed mob
  has no owner.
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

Every mob in the table above can be put on a lead, **tamed or not**, so that
a mob can be led home and tamed there, or a tamed one moved. Vanilla doesn't
allow leads on monsters. Leads work as usual otherwise: shears cut them, and
a lead breaks if pulled too far.

To check in testing: a lead on a flyer (ghast, phantom) and on a mob that
teleports (enderman) or hides (shulker). If one of them misbehaves on a lead,
we decide what to do about that mob then.

## How a tamed mob is shown

- **Hearts** when it's tamed, then **a few green sparkles** (the
  happy-villager particle) above its head now and then, about every 10 to 20
  seconds.
- **A name:** "Tame zombie", "Tame creeper" and so on, shown when you look at
  it, like a name tag. In Polish the word agrees with the mob, using the mob's
  vanilla name: "Oswojony zombie", "Oswojony pająk", "Oswojona wiedźma",
  "Oswojone…" where needed. A name tag can rename it, and it stays tamed.

Both work on LAN and on servers, because particles and names are sent to every
player. Green eyes were considered and left for later: there's no eye texture
for zombies, skeletons or creepers, and a mixin mod can't ship textures.

## How it works (for implementation)

Hook points found in Horizon's source (revision `9f42064c`):

- **Feeding:** a cancellable HEAD hook on
  `horizon_client::game::creatures::Game::interact_mob`, before vanilla's own
  interactions (name tags, leads, buckets).
- **Tamed state:** the entity tag `mord_tamed` in the mob's saved `Tags`
  (`game.persist.raw`), so it's saved with the world and survives conversions.
  `/tag @e[type=zombie] add mord_tamed` tames a mob for testing.
  `Extra.tame` and `Extra.owner` aren't used, because the game saves them only
  for cats, wolves and the like. A HEAD hook on
  `horizon_client::game::creatures::Game::tick_creatures` copies the tagged
  mobs into the runtime's `TAMED` map before each level's mobs tick, since the
  mob engine's hooks see a mob but not its tags.
- **Loot and sparkles:** from the mob tick hook (`tick_end.rs`), which pushes
  `MobEvent::DropItem` and particle events, with a per-mob timer like
  `egg_time`.
- **Hearts and smoke:** queued by the feeding hook and shown from the mob's
  next tick: during a player's turn the game has no LAN hub to send particles
  through, so particles sent there never reach the screen.
- **No teleports:** cancellable HEAD hooks on `mobs::enderman::teleport_to`
  and `mobs::shulker::teleport` refuse a tamed mob's teleport.
- **Lead position:** a cancellable HEAD hook on `mobs::leash::leash_offset`
  hangs the lead at the centre of the mob (half its height).
- **Leads:** a cancellable HEAD hook on `horizon_client_lib::mobs::leash::can_be_leashed` (a RETURN hook is refused there: the function has a nested `use`).
- **Persistence and sunlight:** set `persistent` when taming, and skip
  `burn_undead` for tamed mobs (as for invulnerable undead).
- **Friendliness:** the existing friendly checks in `runtime.rs` also cover
  tamed mobs.
- **Name:** set `custom_name` once on taming, in the game's language at that
  moment.
