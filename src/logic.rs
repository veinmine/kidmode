//! Kid Mode's logic, free of the game: mob groups, settings, taming, loot and the
//! taming hint's pictures. `lib.rs` answers the game's hooks with it. Features:
//! docs/features.md.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use std::sync::atomic::{AtomicBool, Ordering};

/// The mod's id (its settings folder, its namespace) and version, as in
/// `horizon-mod.toml`.
pub const ID: &str = "kidmode";
pub const VERSION: &str = "0.11.0";

/// The languages the mod ships (`assets/kidmode/lang/<code>.json`).
pub const LANGUAGES: &[&str] = &["en_us", "pl_pl"];

// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------

/// One on/off setting: its settings file (under
/// `horizon-mods/.horizon-mod-data/kidmode/`, which also names its label's
/// translation key `kidmode.setting.<key>`) and its default.
pub struct Setting {
    pub key: &'static str,
    pub default: bool,
    value: AtomicBool,
}

impl Setting {
    const fn new(key: &'static str, default: bool) -> Self {
        Setting {
            key,
            default,
            value: AtomicBool::new(default),
        }
    }

    /// The setting as last loaded or set (its default until loaded).
    pub fn on(&self) -> bool {
        self.value.load(Ordering::Acquire)
    }

    pub fn set(&self, on: bool) {
        self.value.store(on, Ordering::Release);
    }

    /// The default unless the file says `true` or `false`.
    pub fn read(&self, stored: Option<String>) -> bool {
        match stored.as_deref().map(str::trim) {
            Some("true") => true,
            Some("false") => false,
            _ => self.default,
        }
    }

    /// The label's translation key.
    pub fn label_key(&self) -> String {
        format!("kidmode.setting.{}", self.key)
    }
}

pub fn stored(on: bool) -> &'static str {
    if on { "true" } else { "false" }
}

pub static FRIENDLY_ENEMIES: Setting = Setting::new("friendly_enemies", true);
pub static FRIENDLY_NEUTRAL: Setting = Setting::new("friendly_neutral_mobs", true);
pub static FRIENDLY_FIGHT_MOBS: Setting = Setting::new("friendly_mobs_fight_mobs", false);
pub static INVULNERABLE_ANIMALS: Setting = Setting::new("invulnerable_animals", true);
pub static INVULNERABLE_ENEMIES: Setting = Setting::new("invulnerable_enemies", false);

pub static INVULNERABLE_DRAGON: Setting = Setting::new("invulnerable_ender_dragon", false);

pub static INVULNERABLE_PLAYER: Setting = Setting::new("invulnerable_player", true);
pub static ALWAYS_SUNNY: Setting = Setting::new("always_sunny", false);
pub static AUTO_SLEEP: Setting = Setting::new("auto_sleep", false);
pub static TAME_FOR_LOOT: Setting = Setting::new("tame_for_loot", true);
pub static TAMING_HINTS: Setting = Setting::new("taming_hints", true);

/// The settings page, in order: button `i` toggles `SETTINGS[i]`.
pub static SETTINGS: [&Setting; 11] = [
    &INVULNERABLE_PLAYER,
    &FRIENDLY_ENEMIES,
    &FRIENDLY_NEUTRAL,
    &FRIENDLY_FIGHT_MOBS,
    &INVULNERABLE_ANIMALS,
    &INVULNERABLE_ENEMIES,
    &INVULNERABLE_DRAGON,
    &ALWAYS_SUNNY,
    &AUTO_SLEEP,
    &TAME_FOR_LOOT,
    &TAMING_HINTS,
];

// ---------------------------------------------------------------------------
// Invulnerable player, Always sunny, Auto sleep
// ---------------------------------------------------------------------------

/// The player is hit (`bypasses`: `/kill`, the void): true to refuse the hit.
pub fn refuse_player_damage(bypasses: bool) -> bool {
    INVULNERABLE_PLAYER.on() && !bypasses
}

/// The overworld clock tick from which a player may sleep (`Player.startSleepInBed`
/// in clear weather): the day has ended.
pub const BEDTIME: i64 = 12_542;

/// Whether Auto sleep skips the night now: it is on, the player is in the
/// overworld (where beds work), and the day has ended.
pub fn auto_sleep_now(dimension: &str, day_time: i64) -> bool {
    AUTO_SLEEP.on() && dimension == "minecraft:overworld" && day_time.rem_euclid(24_000) >= BEDTIME
}

/// How long the cleared weather is set to last (it is cleared again whenever
/// it would turn).
pub const SUNNY_TICKS: i32 = 24_000 * 7;

/// Reads every setting (`get`: a key's stored value). The server reads them
/// every second, and the client when the player changes one; the two share
/// these statics in single player, hence atomics.
pub fn load(get: impl Fn(&str) -> Option<String>) {
    for s in SETTINGS {
        s.set(s.read(get(s.key)));
    }
}

// ---------------------------------------------------------------------------
// Mob groups
// ---------------------------------------------------------------------------

/// Vanilla's hostile mobs: they attack on sight.
pub const HOSTILE: &[&str] = &[
    "blaze",
    "bogged",
    "breeze",
    "cave_spider",
    "creaking",
    "creeper",
    "drowned",
    "elder_guardian",
    "ender_dragon",
    "endermite",
    "evoker",
    "ghast",
    "giant",
    "guardian",
    "hoglin",
    "husk",
    "illusioner",
    "magma_cube",
    "parched",
    "phantom",
    "piglin_brute",
    "pillager",
    "ravager",
    "shulker",
    "silverfish",
    "skeleton",
    "slime",
    "spider",
    "stray",
    "vex",
    "vindicator",
    "warden",
    "witch",
    "wither",
    "wither_skeleton",
    "zoglin",
    "zombie",
    "zombie_nautilus",
    "zombie_villager",
];

/// Vanilla's neutral mobs: they attack only when provoked or in some states.
pub const NEUTRAL: &[&str] = &[
    "bee",
    "dolphin",
    "enderman",
    "goat",
    "iron_golem",
    "llama",
    "panda",
    "piglin",
    "polar_bear",
    "trader_llama",
    "wolf",
    "zombified_piglin",
];

/// Animals: vanilla's passive and neutral creatures, land and water.
pub const ANIMALS: &[&str] = &[
    "armadillo",
    "axolotl",
    "bat",
    "bee",
    "camel",
    "cat",
    "chicken",
    "cod",
    "cow",
    "dolphin",
    "donkey",
    "fox",
    "frog",
    "glow_squid",
    "goat",
    "happy_ghast",
    "horse",
    "llama",
    "mooshroom",
    "mule",
    "nautilus",
    "ocelot",
    "panda",
    "parrot",
    "pig",
    "polar_bear",
    "pufferfish",
    "rabbit",
    "salmon",
    "sheep",
    "skeleton_horse",
    "sniffer",
    "squid",
    "strider",
    "tadpole",
    "trader_llama",
    "tropical_fish",
    "turtle",
    "wolf",
    "zombie_horse",
];

fn vanilla(kind: &str, list: &[&str]) -> bool {
    kind.strip_prefix("minecraft:")
        .is_some_and(|name| list.contains(&name))
}

pub fn is_hostile(kind: &str) -> bool {
    vanilla(kind, HOSTILE)
}
pub fn is_neutral(kind: &str) -> bool {
    vanilla(kind, NEUTRAL)
}
pub fn is_animal(kind: &str) -> bool {
    vanilla(kind, ANIMALS)
}

// ---------------------------------------------------------------------------
// Friendly enemies / Friendly neutral mobs
// ---------------------------------------------------------------------------
// `living_change_target` drops a player (or, unless friendly mobs may fight,
// any) target a friendly mob picks, from goals and brains alike, and the
// player's invulnerability catches whatever still lands.

/// Whether mob type `kind` may not attack players with the current settings.
pub fn friendly_to_players(kind: &str) -> bool {
    (FRIENDLY_ENEMIES.on() && is_hostile(kind)) || (FRIENDLY_NEUTRAL.on() && is_neutral(kind))
}

/// Whether mob `id` of type `kind` may not attack players: its type is
/// friendly, or it is tamed.
pub fn friendly(id: u32, kind: &str) -> bool {
    friendly_to_players(kind) || tamed(id)
}

/// Harmful effects a mob can put on the player directly (not through a
/// projectile). Helpful ones (a dolphin's grace) are kept.
pub const HARMFUL_EFFECTS: &[&str] = &[
    "minecraft:blindness",
    "minecraft:darkness",
    "minecraft:hunger",
    "minecraft:levitation",
    "minecraft:mining_fatigue",
    "minecraft:nausea",
    "minecraft:poison",
    "minecraft:slowness",
    "minecraft:weakness",
    "minecraft:wither",
];

/// Whether a friendly mob of type `kind` may not attack other mobs either.
pub fn spares_mobs(kind: &str) -> bool {
    !FRIENDLY_FIGHT_MOBS.on() && friendly_to_players(kind)
}

/// The same for mob `id`, which also counts when tamed.
pub fn spares_mobs_of(id: u32, kind: &str) -> bool {
    !FRIENDLY_FIGHT_MOBS.on() && friendly(id, kind)
}

// ---------------------------------------------------------------------------
// Invulnerable animals / Invulnerable enemies
// ---------------------------------------------------------------------------
// `entity_invulnerability_check` is vanilla's `isInvulnerableTo` at the start of
// `hurtServer`: a refused hit deals no damage, sets no attacker (no panic, no
// revenge) and gives no knockback. Explosion pushes are hooked separately.

/// Whether mob type `kind` is invulnerable with the current settings.
pub fn invulnerable(kind: &str) -> bool {
    (INVULNERABLE_ANIMALS.on() && is_animal(kind))
        || (INVULNERABLE_ENEMIES.on() && is_enemy_for_invulnerability(kind))
}

/// Invulnerable enemies covers the hostile mobs and the neutral mobs that are
/// not animals (enderman, piglin, zombified piglin, iron golem).
pub fn is_enemy_for_invulnerability(kind: &str) -> bool {
    is_hostile(kind) || (is_neutral(kind) && !is_animal(kind))
}

/// Damage types that still hurt an invulnerable mob or dragon.
pub const BYPASSES_INVULNERABILITY: &[&str] = &["minecraft:generic_kill", "minecraft:out_of_world"];

/// The ender dragon is hit by `damage_type` (none when not known): true to
/// refuse the hit.
pub fn refuse_dragon_damage(damage_type: Option<&str>) -> bool {
    INVULNERABLE_DRAGON.on() && !damage_type.is_some_and(|t| BYPASSES_INVULNERABILITY.contains(&t))
}

/// Mob type `kind` is hit (`bypasses`: `/kill`, the void): true to refuse
/// the hit.
pub fn refuse_damage(kind: &str, bypasses: bool) -> bool {
    !bypasses && invulnerable(kind)
}

/// Whether damage of type `damage_type` still hurts the invulnerable.
pub fn bypasses(damage_type: &str) -> bool {
    BYPASSES_INVULNERABILITY.contains(&damage_type)
}

/// A cap on the per-mob maps, so a long session can't grow them for ever.
const IDS_CAP: usize = 65_536;

// ---------------------------------------------------------------------------
// Tame mobs for loot (docs/taming.md)
// ---------------------------------------------------------------------------
// Feeding a mob its taming item may tame it (1 in 3). A tamed mob carries the
// attachment `kidmode:tamed` (saved with the world and synced to players), or
// the entity tag `kidmode_tamed` (`/tag`); the server copies the tamed mobs
// into `TAMED` each tick, with their loot and sparkle timers.

/// The entity tag a tamed mob carries.
pub const TAMED_TAG: &str = "kidmode_tamed";

/// Mobs that can be tamed: their types, the loot they drop, the items that
/// tame them (all without `minecraft:`).
pub struct Tameable {
    pub mobs: &'static [&'static str],
    pub loot: &'static [&'static str],
    pub food: &'static [&'static str],
}

const RAW_MEAT: &[&str] = &["beef", "porkchop", "chicken", "mutton", "rabbit"];
const FLOWERS: &[&str] = &[
    "dandelion",
    "poppy",
    "blue_orchid",
    "allium",
    "azure_bluet",
    "red_tulip",
    "orange_tulip",
    "white_tulip",
    "pink_tulip",
    "oxeye_daisy",
    "cornflower",
    "lily_of_the_valley",
    "torchflower",
    "open_eyeblossom",
    "closed_eyeblossom",
    "pink_petals",
    "wildflowers",
    "sunflower",
    "lilac",
    "rose_bush",
    "peony",
    "pitcher_plant",
];
const COAL: &[&str] = &["coal", "charcoal"];

pub const TAMEABLE: &[Tameable] = &[
    Tameable {
        mobs: &["zombie", "husk", "zombie_villager"],
        loot: &["rotten_flesh"],
        food: RAW_MEAT,
    },
    Tameable {
        mobs: &["zombified_piglin"],
        loot: &["rotten_flesh", "gold_nugget"],
        food: RAW_MEAT,
    },
    Tameable {
        mobs: &["drowned"],
        loot: &["rotten_flesh", "copper_ingot"],
        food: &["cod", "salmon"],
    },
    Tameable {
        mobs: &["skeleton", "stray", "bogged"],
        loot: &["bone", "arrow"],
        food: &["milk_bucket"],
    },
    Tameable {
        mobs: &["wither_skeleton"],
        loot: &["coal", "bone"],
        food: &["milk_bucket"],
    },
    Tameable {
        mobs: &["creeper"],
        loot: &["gunpowder"],
        food: FLOWERS,
    },
    Tameable {
        mobs: &["spider", "cave_spider"],
        loot: &["string", "spider_eye"],
        food: &["sweet_berries"],
    },
    Tameable {
        mobs: &["enderman"],
        loot: &["ender_pearl"],
        food: &["grass_block"],
    },
    Tameable {
        mobs: &["witch"],
        loot: &[
            "redstone",
            "glowstone_dust",
            "sugar",
            "stick",
            "glass_bottle",
        ],
        food: &["red_mushroom", "brown_mushroom"],
    },
    Tameable {
        mobs: &["slime"],
        loot: &["slime_ball"],
        food: &["sugar"],
    },
    Tameable {
        mobs: &["magma_cube"],
        loot: &["magma_cream"],
        food: COAL,
    },
    Tameable {
        mobs: &["blaze"],
        loot: &["blaze_rod"],
        food: COAL,
    },
    Tameable {
        mobs: &["guardian"],
        loot: &["prismarine_shard", "prismarine_crystals"],
        food: &["tropical_fish"],
    },
    Tameable {
        mobs: &["shulker"],
        loot: &["shulker_shell"],
        food: &["chorus_fruit"],
    },
    Tameable {
        mobs: &["breeze"],
        loot: &["breeze_rod"],
        food: &["feather"],
    },
    Tameable {
        mobs: &["hoglin"],
        loot: &["leather", "porkchop"],
        food: &["crimson_fungus"],
    },
    Tameable {
        mobs: &["ghast"],
        loot: &["ghast_tear", "gunpowder"],
        food: &["snowball"],
    },
    Tameable {
        mobs: &["phantom"],
        loot: &["phantom_membrane"],
        food: &["glow_berries"],
    },
    // Animals (an empty loot list: tamed for company only).
    Tameable {
        mobs: &["cow", "mooshroom"],
        loot: &["leather", "beef"],
        food: &["golden_carrot"],
    },
    Tameable {
        mobs: &["chicken"],
        loot: &["feather", "chicken"],
        food: &["golden_carrot"],
    },
    Tameable {
        mobs: &["sheep"],
        loot: &["mutton"],
        food: &["golden_carrot"],
    },
    Tameable {
        mobs: &["pig"],
        loot: &["porkchop"],
        food: &["golden_carrot"],
    },
    Tameable {
        mobs: &["goat"],
        loot: &[],
        food: &["golden_carrot"],
    },
    Tameable {
        mobs: &["rabbit"],
        loot: &["rabbit_hide", "rabbit"],
        food: &["apple"],
    },
    Tameable {
        mobs: &["polar_bear"],
        loot: &["cod", "salmon"],
        food: &["cod", "salmon"],
    },
    Tameable {
        mobs: &["panda"],
        loot: &["bamboo"],
        food: &["sugar_cane"],
    },
    Tameable {
        mobs: &["fox"],
        loot: &["sweet_berries"],
        food: &["chicken"],
    },
    Tameable {
        mobs: &["bee"],
        loot: &["honeycomb"],
        food: &["sugar"],
    },
    // The game picks the froglight of the frog's colour (`frog_loot`).
    Tameable {
        mobs: &["frog"],
        loot: &["ochre_froglight"],
        food: &["spider_eye"],
    },
    Tameable {
        mobs: &["armadillo"],
        loot: &[],
        food: &["apple"],
    },
    Tameable {
        mobs: &["sniffer"],
        loot: &[],
        food: &["moss_block"],
    },
    Tameable {
        mobs: &["camel"],
        loot: &[],
        food: &["hay_block"],
    },
    Tameable {
        mobs: &["strider"],
        loot: &["string"],
        food: &["crimson_fungus"],
    },
    Tameable {
        mobs: &["bat"],
        loot: &[],
        food: &["melon_slice"],
    },
    Tameable {
        mobs: &["squid"],
        loot: &["ink_sac"],
        food: &["kelp"],
    },
    Tameable {
        mobs: &["glow_squid"],
        loot: &["glow_ink_sac"],
        food: &["kelp"],
    },
    Tameable {
        mobs: &["dolphin"],
        loot: &["cod"],
        food: &["kelp"],
    },
    Tameable {
        mobs: &["cod"],
        loot: &["cod", "bone_meal"],
        food: &["kelp"],
    },
    Tameable {
        mobs: &["salmon"],
        loot: &["salmon", "bone_meal"],
        food: &["kelp"],
    },
    Tameable {
        mobs: &["tropical_fish"],
        loot: &["tropical_fish", "bone_meal"],
        food: &["kelp"],
    },
    Tameable {
        mobs: &["pufferfish"],
        loot: &["pufferfish", "bone_meal"],
        food: &["kelp"],
    },
    Tameable {
        mobs: &["turtle"],
        loot: &["seagrass", "turtle_scute"],
        food: &["kelp"],
    },
    Tameable {
        mobs: &["axolotl"],
        loot: &[],
        food: &["kelp"],
    },
];

/// Vanilla's pets that drop loot once tamed the vanilla way (their tame flag),
/// and what they drop. Wolves and ocelots drop nothing.
pub const PET_LOOT: &[(&[&str], &[&str])] = &[
    (&["cat"], &["string"]),
    (&["parrot"], &["feather"]),
    (
        &["horse", "donkey", "mule", "llama", "trader_llama"],
        &["leather"],
    ),
    (&["skeleton_horse"], &["bone"]),
    (&["zombie_horse"], &["rotten_flesh"]),
];

/// The loot of vanilla pet type `kind` once tamed, if it drops any.
pub fn pet_loot(kind: &str) -> Option<&'static [&'static str]> {
    let name = kind.strip_prefix("minecraft:")?;
    PET_LOOT
        .iter()
        .find(|(mobs, _)| mobs.contains(&name))
        .map(|(_, loot)| *loot)
}

/// What a tamed mob of type `kind` drops: Kid Mode's table or a vanilla pet's.
pub fn loot_of(kind: &str) -> Option<&'static [&'static str]> {
    tameable(kind).map(|t| t.loot).or_else(|| pet_loot(kind))
}

/// The froglight a tamed frog of variant `variant` drops, as a frog eating a
/// magma cube makes: ochre (temperate), verdant (cold), pearlescent (warm).
pub fn frog_loot(variant: &str) -> &'static str {
    match variant.strip_prefix("minecraft:").unwrap_or(variant) {
        "cold" => "minecraft:verdant_froglight",
        "warm" => "minecraft:pearlescent_froglight",
        _ => "minecraft:ochre_froglight",
    }
}

/// The taming entry of mob type `kind`, if it can be tamed.
pub fn tameable(kind: &str) -> Option<&'static Tameable> {
    let name = kind.strip_prefix("minecraft:")?;
    TAMEABLE.iter().find(|t| t.mobs.contains(&name))
}

/// Whether `item` tames mob type `kind`.
pub fn tames(kind: &str, item: &str) -> bool {
    let Some(item) = item.strip_prefix("minecraft:") else {
        return false;
    };
    TAME_FOR_LOOT.on() && tameable(kind).is_some_and(|t| t.food.contains(&item))
}

/// What a taming item leaves in the hand once used (a milk bucket's bucket).
pub fn leftover(item: &str) -> Option<&'static str> {
    (item == "minecraft:milk_bucket").then_some("minecraft:bucket")
}

/// Whether mob type `kind` can be put on a lead because of this feature.
/// Animals keep vanilla's leads (no lead on a fish, squid or bat).
pub fn leashable(kind: &str) -> bool {
    TAME_FOR_LOOT.on() && tameable(kind).is_some() && !is_animal(kind)
}

/// Whether a lead on mob type `kind` hangs from the middle of its body (half
/// its height, on its centre) instead of vanilla's eye height. Animals keep
/// vanilla's.
pub fn lead_at_middle(kind: &str) -> bool {
    leashable(kind)
}

/// The tamed mobs by id, and each one's loot and sparkle countdowns.
struct Tamed {
    ids: HashMap<u32, Timers>,
}

#[derive(Clone, Copy)]
struct Timers {
    loot: i32,
    sparkle: i32,
}

static TAMED: Mutex<Option<Tamed>> = Mutex::new(None);

/// Ticks until the next loot: 6000 to 12000, as a chicken's egg.
pub const LOOT_MIN: i32 = 6000;
pub const LOOT_SPREAD: i32 = 6000;
/// Ticks until the next sparkles: 200 to 400.
pub const SPARKLE_MIN: i32 = 200;
pub const SPARKLE_SPREAD: i32 = 200;

/// Whether mob `id` is tamed (and the feature on).
pub fn tamed(id: u32) -> bool {
    TAME_FOR_LOOT.on()
        && TAMED
            .lock()
            .ok()
            .is_some_and(|t| t.as_ref().is_some_and(|t| t.ids.contains_key(&id)))
}

/// Mob `id` is tamed (`true`) or not, as the server sees it. `roll(n)` is a random number below `n`.
pub fn set_tamed(id: u32, is_tamed: bool, roll: impl FnOnce(i32) -> i32) {
    let Ok(mut t) = TAMED.lock() else { return };
    let t = t.get_or_insert_with(|| Tamed {
        ids: HashMap::new(),
    });
    if !is_tamed {
        t.ids.remove(&id);
    } else if !t.ids.contains_key(&id) {
        if t.ids.len() >= IDS_CAP {
            t.ids.clear();
        }
        let loot = LOOT_MIN + roll(LOOT_SPREAD);
        t.ids.insert(
            id,
            Timers {
                loot,
                sparkle: SPARKLE_MIN,
            },
        );
    }
}

/// What a tamed mob does this tick.
#[derive(Debug, Default, PartialEq)]
pub struct TamedTick {
    /// The item it drops (`minecraft:` id).
    pub loot: Option<String>,
    /// Whether it shows its sparkles.
    pub sparkle: bool,
}

/// Mob `id` of type `kind` ticks (`adult`: not a baby). `roll(n)` is a random
/// number below `n`.
pub fn tamed_tick(id: u32, kind: &str, adult: bool, mut roll: impl FnMut(i32) -> i32) -> TamedTick {
    let mut out = TamedTick::default();
    if !TAME_FOR_LOOT.on() {
        return out;
    }
    let Ok(mut t) = TAMED.lock() else { return out };
    let Some(timers) = t.as_mut().and_then(|t| t.ids.get_mut(&id)) else {
        return out;
    };
    timers.sparkle -= 1;
    if timers.sparkle <= 0 {
        timers.sparkle = SPARKLE_MIN + roll(SPARKLE_SPREAD);
        out.sparkle = true;
    }
    if adult {
        timers.loot -= 1;
        if timers.loot <= 0 {
            timers.loot = LOOT_MIN + roll(LOOT_SPREAD);
            if let Some(loot) = loot_of(kind).filter(|l| !l.is_empty()) {
                let i = roll(loot.len() as i32).clamp(0, loot.len() as i32 - 1) as usize;
                out.loot = Some(format!("minecraft:{}", loot[i]));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Taming hints (docs/features.md)
// ---------------------------------------------------------------------------
// Looking at a mob that can be tamed and isn't yet, from up to `HINT_DISTANCE`
// blocks, shows a thought bubble over its head with its taming item. Drawn on
// the client, after the mob (`render_living_post`, in `hint.rs`).

/// How far away (blocks, from the eyes) a looked-at mob shows its bubble.
pub const HINT_DISTANCE: f64 = 8.0;

/// Ticks each taming item shows for when a mob takes several (1 second).
pub const HINT_CYCLE: i64 = 20;

/// Vanilla's tameable mobs and what tames them (without `minecraft:`). Horses,
/// donkeys, mules and llamas are tamed by riding them: they show a saddle.
pub const VANILLA_TAMEABLE: &[(&str, &[&str])] = &[
    ("wolf", &["bone"]),
    ("cat", &["cod", "salmon"]),
    ("ocelot", &["cod", "salmon"]),
    (
        "parrot",
        &[
            "wheat_seeds",
            "melon_seeds",
            "pumpkin_seeds",
            "beetroot_seeds",
            "torchflower_seeds",
            "pitcher_pod",
        ],
    ),
    ("nautilus", &["pufferfish"]),
    ("horse", &["saddle"]),
    ("donkey", &["saddle"]),
    ("mule", &["saddle"]),
    ("llama", &["saddle"]),
    ("trader_llama", &["saddle"]),
];

/// The items that tame mob type `kind` in vanilla.
pub fn vanilla_taming_items(kind: &str) -> Option<&'static [&'static str]> {
    let name = kind.strip_prefix("minecraft:")?;
    VANILLA_TAMEABLE
        .iter()
        .find(|(m, _)| *m == name)
        .map(|(_, items)| *items)
}

/// Whether a mob of type `kind` may show a bubble at all: a cheap first check
/// before looking the mob up.
pub fn may_hint(kind: &str) -> bool {
    TAMING_HINTS.on()
        && (vanilla_taming_items(kind).is_some()
            || (TAME_FOR_LOOT.on() && tameable(kind).is_some()))
}

/// What the client knows about a mob's taming.
#[derive(Clone, Copy, Debug, Default)]
pub struct TameState {
    /// Vanilla's tame flag (wolves, cats, parrots, nautili, horses, llamas).
    pub tame: bool,
    /// An ocelot's trust.
    pub trusting: bool,
    /// An angry wolf can't be tamed.
    pub angry: bool,
    pub baby: bool,
    /// The server says the mod tamed it (its synced `kidmode:tamed`).
    pub synced_tamed: bool,
    /// It says another player tamed it (not the one looking).
    pub tamed_by_other: bool,
}

/// What a mob's bubble shows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hint {
    /// Not tamed yet: the items that tame it, in turn.
    Items(&'static [&'static str]),
    /// Tamed: a heart (to its tamer, or when nobody is known as its tamer).
    Tamed,
    /// Tamed by another player: their head.
    TamedBy,
}

/// The bubble over mob `id` of type `kind`, or None when it shows none
/// (setting off, can't be tamed, or can't be tamed now).
pub fn hint(kind: &str, id: u32, state: TameState) -> Option<Hint> {
    if !TAMING_HINTS.on() {
        return None;
    }
    if let Some(items) = vanilla_taming_items(kind) {
        let name = kind.strip_prefix("minecraft:").unwrap_or(kind);
        let tamed = if name == "ocelot" {
            state.trusting
        } else {
            state.tame
        };
        if tamed {
            return Some(Hint::Tamed);
        }
        let now = match name {
            "wolf" => !state.angry,
            // A baby can't be ridden, and a baby nautilus can't be tamed.
            "nautilus" | "horse" | "donkey" | "mule" | "llama" | "trader_llama" => !state.baby,
            _ => true,
        };
        return now.then_some(Hint::Items(items));
    }
    if !TAME_FOR_LOOT.on() {
        return None;
    }
    let entry = tameable(kind)?;
    Some(if state.tamed_by_other {
        Hint::TamedBy
    } else if state.synced_tamed || tamed(id) {
        Hint::Tamed
    } else {
        Hint::Items(entry.food)
    })
}

/// The item shown at game time `time` (`minecraft:` id): each in turn, for
/// `HINT_CYCLE` ticks.
pub fn hint_item(items: &[&str], time: i64) -> String {
    if items.is_empty() {
        return String::new();
    }
    let i = (time.div_euclid(HINT_CYCLE)).rem_euclid(items.len() as i64) as usize;
    format!("minecraft:{}", items[i])
}

/// How long the bubble takes to appear, how long it stays once the player
/// looks away, and how long it then takes to fade out.
pub const HINT_FADE_IN: Duration = Duration::from_millis(150);
pub const HINT_LINGER: Duration = Duration::from_millis(1500);
pub const HINT_FADE_OUT: Duration = Duration::from_millis(400);
/// A frame gap longer than this means the player looked away (or the mob
/// left the view) and came back.
const HINT_GAP: Duration = Duration::from_millis(100);

/// The bubbles showing, by mob id: when each started (shifted back on a
/// return, to fade in from where it was) and when it was last looked at.
static HINT_SEEN: Mutex<Option<HashMap<u32, (Instant, Instant)>>> = Mutex::new(None);

/// How opaque mob `id`'s bubble is at `now`, 0 to 1, as the frame draws it
/// (`looked`: the player looks at it now).
pub fn hint_alpha(id: u32, looked: bool, now: Instant) -> f32 {
    let Ok(mut seen) = HINT_SEEN.lock() else {
        return f32::from(u8::from(looked));
    };
    let seen = seen.get_or_insert_with(HashMap::new);
    let entry = seen.get(&id).copied();
    let before = entry.map_or(0.0, |e| fade(e, now));
    if looked {
        let start = match entry {
            Some((start, last)) if now.saturating_duration_since(last) <= HINT_GAP => start,
            _ => now.checked_sub(HINT_FADE_IN.mul_f32(before)).unwrap_or(now),
        };
        if seen.len() >= IDS_CAP {
            seen.clear();
        }
        seen.insert(id, (start, now));
        return fade((start, now), now);
    }
    if entry.is_some() && before <= 0.0 {
        seen.remove(&id);
    }
    before
}

/// A bubble's opacity at `now`: fading in from `start`, full until
/// `HINT_LINGER` after `last`, then fading out.
fn fade((start, last): (Instant, Instant), now: Instant) -> f32 {
    let shown =
        (last.saturating_duration_since(start).as_secs_f32() / HINT_FADE_IN.as_secs_f32()).min(1.0);
    let away = now.saturating_duration_since(last);
    if away <= HINT_LINGER {
        return shown;
    }
    let out = (away - HINT_LINGER).as_secs_f32() / HINT_FADE_OUT.as_secs_f32();
    (shown * (1.0 - out)).max(0.0)
}

/// The bubble, a pixel a character: `O` its outline, `W` its inside, `.`
/// nothing. The cloud on top holds the item; the dots trail down to the head.
pub const BUBBLE: [&str; 25] = [
    "....OOOOOOOOOO....",
    "..OOWWWWWWWWWWOO..",
    ".OWWWWWWWWWWWWWWO.",
    ".OWWWWWWWWWWWWWWO.",
    "OWWWWWWWWWWWWWWWWO",
    "OWWWWWWWWWWWWWWWWO",
    "OWWWWWWWWWWWWWWWWO",
    "OWWWWWWWWWWWWWWWWO",
    "OWWWWWWWWWWWWWWWWO",
    "OWWWWWWWWWWWWWWWWO",
    "OWWWWWWWWWWWWWWWWO",
    "OWWWWWWWWWWWWWWWWO",
    ".OWWWWWWWWWWWWWWO.",
    ".OWWWWWWWWWWWWWWO.",
    "..OOWWWWWWWWWWOO..",
    "....OOOOOOOOOO....",
    "..................",
    ".....OOOO.........",
    "....OWWWWO........",
    "....OWWWWO........",
    ".....OOOO.........",
    "..................",
    "......OOO.........",
    "......OWO.........",
    "......OOO.........",
];
/// The bubble's width and height, in its pixels.
pub const BUBBLE_W: usize = 18;
pub const BUBBLE_H: usize = BUBBLE.len();
/// Where the item's centre sits (pixels from the bubble's top left), and its size.
pub const BUBBLE_ICON_CENTRE: [f32; 2] = [9.0, 8.0];
pub const BUBBLE_ICON: f32 = 12.0;
/// A bubble pixel's size in blocks: the bubble is 0.72 blocks wide.
pub const BUBBLE_PIXEL: f32 = 0.04;
/// Colours as the vertices take them (0xAABBGGRR): outline and inside.
pub const BUBBLE_OUTLINE: u32 = 0xFF3A_3A3A;
pub const BUBBLE_FILL: u32 = 0xFFFF_FFFF;

/// The heart a tamed mob's bubble shows: `O` outline, `R` red, `W` shine.
pub const HEART: [&str; 10] = [
    ".OOO...OOO.",
    "ORRRO.ORRRO",
    "ORWRROORRRO",
    "ORRRRRRRRRO",
    "ORRRRRRRRRO",
    ".ORRRRRRRO.",
    "..ORRRRRO..",
    "...ORRRO...",
    "....ORO....",
    ".....O.....",
];
pub const HEART_OUTLINE: u32 = 0xFF10_1050;
pub const HEART_RED: u32 = 0xFF20_20E0;
pub const HEART_SHINE: u32 = 0xFFC0_C0FF;

/// The bubble as runs of one colour: (row, first column, column after the
/// last, colour), rows counted from the top.
pub fn bubble_runs() -> Vec<(usize, usize, usize, u32)> {
    runs(&BUBBLE)
}

/// The heart as runs, as `bubble_runs`.
pub fn heart_runs() -> Vec<(usize, usize, usize, u32)> {
    runs(&HEART)
}

/// A picture (`O` outline, `W` white or shine, `R` red) as runs of one colour.
fn runs(picture: &[&str]) -> Vec<(usize, usize, usize, u32)> {
    let heart = picture.iter().any(|l| l.contains('R'));
    let mut runs = Vec::new();
    for (row, line) in picture.iter().enumerate() {
        let bytes = line.as_bytes();
        let mut x = 0;
        while x < bytes.len() {
            let c = bytes[x];
            let start = x;
            while x < bytes.len() && bytes[x] == c {
                x += 1;
            }
            let color = match (c, heart) {
                (b'O', false) => BUBBLE_OUTLINE,
                (b'W', false) => BUBBLE_FILL,
                (b'O', true) => HEART_OUTLINE,
                (b'R', true) => HEART_RED,
                (b'W', true) => HEART_SHINE,
                _ => continue,
            };
            runs.push((row, start, x, color));
        }
    }
    runs
}

/// How high over the mob's feet the bubble's bottom sits: above its head, and
/// above its name when it has one.
pub fn bubble_lift(height: f32, named: bool) -> f32 {
    height + if named { 0.6 } else { 0.25 }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The settings are global, so the tests that change them run in one test.
    #[test]
    fn behaviour_follows_the_settings() {
        assert!(
            FRIENDLY_ENEMIES.default && FRIENDLY_NEUTRAL.default && !FRIENDLY_FIGHT_MOBS.default
        );
        assert!(INVULNERABLE_ANIMALS.default && !INVULNERABLE_ENEMIES.default);
        assert!(!INVULNERABLE_DRAGON.default);
        assert!(INVULNERABLE_PLAYER.default && !ALWAYS_SUNNY.default && !AUTO_SLEEP.default);
        assert!(refuse_player_damage(false) && !refuse_player_damage(true));
        assert!(!auto_sleep_now("minecraft:overworld", 13_000));
        AUTO_SLEEP.set(true);
        assert!(auto_sleep_now("minecraft:overworld", 24_000 * 3 + 13_000));
        assert!(!auto_sleep_now("minecraft:overworld", 6_000));
        assert!(!auto_sleep_now("minecraft:the_nether", 13_000));

        assert!(friendly_to_players("minecraft:skeleton"));
        assert!(friendly_to_players("minecraft:zombified_piglin"));
        assert!(!friendly_to_players("minecraft:cow"));
        assert!(spares_mobs("minecraft:iron_golem"));
        FRIENDLY_FIGHT_MOBS.set(true);
        assert!(!spares_mobs("minecraft:iron_golem"));
        FRIENDLY_FIGHT_MOBS.set(false);
        FRIENDLY_NEUTRAL.set(false);
        assert!(!friendly_to_players("minecraft:zombified_piglin"));
        assert!(!spares_mobs("minecraft:iron_golem"));
        assert!(friendly_to_players("minecraft:creeper"));
        FRIENDLY_ENEMIES.set(false);
        assert!(!friendly_to_players("minecraft:creeper"));

        assert!(refuse_damage("minecraft:cow", false));
        assert!(!refuse_damage("minecraft:cow", true));
        assert!(!refuse_damage("minecraft:zombie", false));
        INVULNERABLE_ENEMIES.set(true);
        assert!(refuse_damage("minecraft:zombie", false));
        assert!(refuse_damage("minecraft:zombified_piglin", false));
        assert!(refuse_damage("minecraft:iron_golem", false));
        INVULNERABLE_ANIMALS.set(false);
        assert!(!refuse_damage("minecraft:wolf", false));
        assert!(!refuse_damage("minecraft:cow", false));
        assert!(!refuse_damage("minecraft:player", false));
        assert!(bypasses("minecraft:out_of_world") && !bypasses("minecraft:fall"));

        assert!(!refuse_dragon_damage(Some("minecraft:arrow")));
        INVULNERABLE_DRAGON.set(true);
        assert!(refuse_dragon_damage(Some("minecraft:arrow")));
        assert!(refuse_dragon_damage(None));
        assert!(!refuse_dragon_damage(Some("minecraft:generic_kill")));
    }

    #[test]
    fn settings_read_their_files() {
        assert!(FRIENDLY_ENEMIES.read(None));
        assert!(FRIENDLY_ENEMIES.read(Some("garbage".into())));
        assert!(!FRIENDLY_ENEMIES.read(Some("false\n".into())));
        assert!(!INVULNERABLE_ENEMIES.read(None));
        assert!(INVULNERABLE_ENEMIES.read(Some("true".into())));
        assert_eq!(stored(true), "true");
    }

    /// The keys of a language file, in order (one `"key": "value"` a line).
    fn lang_keys(code: &str) -> Vec<String> {
        let path = format!(
            "{}/assets/kidmode/lang/{code}.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let file = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        file.lines()
            .filter_map(|l| l.trim().strip_prefix('"')?.split_once('"'))
            .map(|(k, _)| k.to_owned())
            .collect()
    }

    #[test]
    fn translations_are_complete() {
        let english = lang_keys("en_us");
        for code in LANGUAGES {
            assert_eq!(
                lang_keys(code),
                english,
                "{code} has other keys than en_us, or in another order"
            );
        }
        for s in SETTINGS {
            assert!(english.contains(&s.label_key()), "{} has no label", s.key);
        }
        assert_eq!(AUTO_SLEEP.label_key(), "kidmode.setting.auto_sleep");
    }

    #[test]
    fn taming() {
        assert!(TAME_FOR_LOOT.default);
        assert!(tames("minecraft:zombie", "minecraft:beef"));
        assert!(tames("minecraft:creeper", "minecraft:poppy"));
        assert!(tames("minecraft:cow", "minecraft:golden_carrot"));
        assert!(!tames("minecraft:cow", "minecraft:wheat"));
        assert!(!tames("minecraft:villager", "minecraft:beef"));
        assert!(!tames("minecraft:zombie", "beef"));
        assert!(leashable("minecraft:ghast") && !leashable("minecraft:villager"));
        assert!(tames("minecraft:sheep", "minecraft:golden_carrot"));
        assert!(tames("minecraft:dolphin", "minecraft:kelp"));
        assert!(!tames("minecraft:sheep", "minecraft:wheat"));
        assert!(!tames("minecraft:wolf", "minecraft:golden_carrot"));
        // Animals keep vanilla's leads.
        assert!(!leashable("minecraft:cod") && !leashable("minecraft:cow"));
        assert_eq!(loot_of("minecraft:cow"), Some(&["leather", "beef"][..]));
        assert_eq!(loot_of("minecraft:cat"), Some(&["string"][..]));
        assert_eq!(loot_of("minecraft:goat"), Some(&[][..]));
        assert_eq!(loot_of("minecraft:wolf"), None);
        assert_eq!(
            frog_loot("minecraft:warm"),
            "minecraft:pearlescent_froglight"
        );
        assert_eq!(frog_loot("minecraft:cold"), "minecraft:verdant_froglight");
        assert_eq!(
            frog_loot("minecraft:temperate"),
            "minecraft:ochre_froglight"
        );
        for (mobs, loot) in PET_LOOT {
            assert!(!loot.is_empty());
            for m in *mobs {
                assert!(tameable(&format!("minecraft:{m}")).is_none(), "{m} in both");
            }
        }
        assert_eq!(leftover("minecraft:milk_bucket"), Some("minecraft:bucket"));
        assert_eq!(leftover("minecraft:beef"), None);
        for t in TAMEABLE {
            assert!(!t.food.is_empty());
            for m in t.mobs {
                assert_eq!(
                    TAMEABLE.iter().filter(|o| o.mobs.contains(m)).count(),
                    1,
                    "{m} twice"
                );
            }
        }
        assert!(!tamed(500));
        set_tamed(500, true, |_| 0);
        assert!(tamed(500));
        // No loot before the timer runs out, sparkles after SPARKLE_MIN ticks.
        let mut sparkles = 0;
        for _ in 0..LOOT_MIN - 1 {
            let t = tamed_tick(500, "minecraft:creeper", true, |_| 0);
            assert_eq!(t.loot, None);
            sparkles += usize::from(t.sparkle);
        }
        assert_eq!(sparkles, ((LOOT_MIN - 1) / SPARKLE_MIN) as usize);
        let t = tamed_tick(500, "minecraft:creeper", true, |_| 0);
        assert_eq!(t.loot.as_deref(), Some("minecraft:gunpowder"));
        // A baby drops nothing.
        set_tamed(501, true, |_| 0);
        for _ in 0..LOOT_MIN + 1 {
            assert_eq!(tamed_tick(501, "minecraft:zombie", false, |_| 0).loot, None);
        }
        // A tamed animal with nothing to drop drops nothing.
        set_tamed(502, true, |_| 0);
        for _ in 0..LOOT_MIN + 1 {
            assert_eq!(tamed_tick(502, "minecraft:goat", true, |_| 0).loot, None);
        }
        set_tamed(503, true, |_| 0);
        let mut loot = None;
        for _ in 0..LOOT_MIN {
            loot = tamed_tick(503, "minecraft:horse", true, |_| 0).loot;
        }
        assert_eq!(loot.as_deref(), Some("minecraft:leather"));
        set_tamed(500, false, |_| 0);
        assert!(!tamed(500));
        assert_eq!(
            tamed_tick(500, "minecraft:creeper", true, |_| 0),
            TamedTick::default()
        );
        assert!(friendly(501, "minecraft:cow"));
        assert!(lead_at_middle("minecraft:zombie") && !lead_at_middle("minecraft:cow"));
    }

    #[test]
    fn taming_hints() {
        assert!(TAMING_HINTS.default);
        let wild = TameState::default();
        assert!(may_hint("minecraft:wolf") && may_hint("minecraft:zombie"));
        assert!(may_hint("minecraft:horse") && !may_hint("minecraft:villager"));
        let items = |i: &'static [&'static str]| Some(Hint::Items(i));
        assert_eq!(hint("minecraft:wolf", 1, wild), items(&["bone"]));
        let tame = TameState { tame: true, ..wild };
        assert_eq!(hint("minecraft:wolf", 1, tame), Some(Hint::Tamed));
        assert_eq!(hint("minecraft:horse", 1, tame), Some(Hint::Tamed));
        let angry = TameState {
            angry: true,
            ..wild
        };
        assert_eq!(hint("minecraft:wolf", 1, angry), None);
        let trusting = TameState {
            trusting: true,
            ..wild
        };
        assert_eq!(hint("minecraft:ocelot", 1, trusting), Some(Hint::Tamed));
        assert_eq!(hint("minecraft:ocelot", 1, tame), items(&["cod", "salmon"]));
        let baby = TameState { baby: true, ..wild };
        assert_eq!(hint("minecraft:nautilus", 1, baby), None);
        assert_eq!(hint("minecraft:horse", 1, baby), None);
        assert_eq!(hint("minecraft:llama", 1, wild), items(&["saddle"]));
        assert!(hint("minecraft:wolf", 1, baby).is_some());
        assert_eq!(
            hint("minecraft:enderman", 600, wild),
            items(&["grass_block"])
        );
        set_tamed(600, true, |_| 0);
        assert_eq!(hint("minecraft:enderman", 600, wild), Some(Hint::Tamed));
        set_tamed(600, false, |_| 0);
        let synced = TameState {
            synced_tamed: true,
            ..wild
        };
        assert_eq!(hint("minecraft:enderman", 600, synced), Some(Hint::Tamed));
        let theirs = TameState {
            tamed_by_other: true,
            ..synced
        };
        assert_eq!(hint("minecraft:enderman", 600, theirs), Some(Hint::TamedBy));
        // Vanilla mobs show the heart: their owner isn't sent to players.
        let tame_wolf = TameState {
            tame: true,
            tamed_by_other: true,
            ..wild
        };
        assert_eq!(hint("minecraft:wolf", 1, tame_wolf), Some(Hint::Tamed));
        assert_eq!(hint("minecraft:villager", 600, wild), None);

        let items = ["cod", "salmon"];
        assert_eq!(hint_item(&items, 0), "minecraft:cod");
        assert_eq!(hint_item(&items, HINT_CYCLE - 1), "minecraft:cod");
        assert_eq!(hint_item(&items, HINT_CYCLE), "minecraft:salmon");
        assert_eq!(hint_item(&items, 2 * HINT_CYCLE), "minecraft:cod");
        assert_eq!(hint_item(&items, -1), "minecraft:salmon");
        assert_eq!(hint_item(&[], 5), "");
        for (m, items) in VANILLA_TAMEABLE {
            assert!(!items.is_empty());
            assert!(tameable(&format!("minecraft:{m}")).is_none(), "{m} in both");
        }

        assert!(BUBBLE.iter().all(|l| l.len() == BUBBLE_W));
        let runs = bubble_runs();
        assert_eq!(runs[0], (0, 4, 14, BUBBLE_OUTLINE));
        assert!(runs.contains(&(23, 7, 8, BUBBLE_FILL)));
        assert!(runs.iter().all(|r| r.1 < r.2 && r.2 <= BUBBLE_W));
        assert!(bubble_lift(2.0, true) > bubble_lift(2.0, false));
        assert!(HEART.iter().all(|l| l.len() == HEART[0].len()));
        let heart = heart_runs();
        assert_eq!(heart[0], (0, 1, 4, HEART_OUTLINE));
        assert!(heart.contains(&(2, 2, 3, HEART_SHINE)));
        assert!(heart.contains(&(9, 5, 6, HEART_OUTLINE)));

        TAMING_HINTS.set(false);
        assert!(!may_hint("minecraft:wolf"));
        assert_eq!(hint("minecraft:wolf", 1, wild), None);
        TAMING_HINTS.set(true);
    }

    #[test]
    fn hint_fades() {
        let t0 = Instant::now();
        let ms = |n: u64| t0 + Duration::from_millis(n);
        assert_eq!(hint_alpha(900, false, ms(0)), 0.0);
        assert_eq!(hint_alpha(900, true, ms(0)), 0.0);
        assert!((hint_alpha(900, true, ms(75)) - 0.5).abs() < 0.01);
        assert_eq!(hint_alpha(900, true, ms(150)), 1.0);
        assert_eq!(hint_alpha(900, true, ms(200)), 1.0);
        // Looked away: stays, then fades out.
        assert_eq!(hint_alpha(900, false, ms(1000)), 1.0);
        assert_eq!(hint_alpha(900, false, ms(1700)), 1.0);
        assert!((hint_alpha(900, false, ms(1900)) - 0.5).abs() < 0.01);
        // Looked back mid-fade: fades in again from there.
        let back = hint_alpha(900, true, ms(1900));
        assert!((back - 0.5).abs() < 0.02, "{back}");
        assert_eq!(hint_alpha(900, true, ms(2000)), 1.0);
        assert_eq!(hint_alpha(900, false, ms(2000 + 1500 + 400)), 0.0);
        assert_eq!(hint_alpha(900, false, ms(5000)), 0.0);
    }

    #[test]
    fn groups() {
        assert!(is_hostile("minecraft:zombie") && !is_hostile("minecraft:wolf"));
        assert!(is_neutral("minecraft:enderman") && !is_neutral("minecraft:zombie"));
        assert!(is_animal("minecraft:cow") && is_animal("minecraft:wolf"));
        assert!(!is_animal("minecraft:iron_golem") && !is_animal("othermod:cow"));
        for name in HOSTILE {
            assert!(
                !NEUTRAL.contains(name),
                "{name} is both hostile and neutral"
            );
        }
    }
}
