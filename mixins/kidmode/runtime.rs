// Kid Mode (mod id `kidmode`), a native Horizon mod by veinmine. This file is both the mod's Cargo
// library (its tests: `cargo test`) and the module the bundle adds to the game
// as `crate::game::kidmode`. Module payloads cannot use inner attributes or `//!`
// comments, external `mod` files or `include!`. Features: docs/features.md.

use std::collections::HashMap;
use std::sync::Mutex;

use std::sync::atomic::{AtomicBool, Ordering};

/// The mod's id (its settings folder) and version. Its name and description
/// are translated: the `kidmode.name` and `kidmode.description` keys below.
pub const ID: &str = "kidmode";
pub const VERSION: &str = "0.7.0";

// ---------------------------------------------------------------------------
// Translations
// ---------------------------------------------------------------------------

/// Every text Options > Mods shows for the mod, by translation key, in the
/// game's language codes. `mixins/kidmode/lang/<code>.json` holds the same, as
/// vanilla-style language files (a test keeps them equal). A key a language
/// lacks reads in English. `options.on` and `options.off` are vanilla's own
/// keys, so the toggles read as the game's other ON/OFF buttons.
pub const EN_US: &[(&str, &str)] = &[
    ("kidmode.name", "Kid Mode"),
    (
        "kidmode.description",
        "A gentler world: friendly and invulnerable mobs, an invulnerable player, sunny days and nights that pass by themselves.",
    ),
    ("kidmode.setting.invulnerable_player", "Invulnerable player"),
    ("kidmode.setting.friendly_enemies", "Friendly enemies"),
    (
        "kidmode.setting.friendly_neutral_mobs",
        "Friendly neutral mobs",
    ),
    (
        "kidmode.setting.friendly_mobs_fight_mobs",
        "Friendly mobs fight other mobs",
    ),
    (
        "kidmode.setting.invulnerable_animals",
        "Invulnerable animals",
    ),
    (
        "kidmode.setting.invulnerable_enemies",
        "Invulnerable enemies",
    ),
    (
        "kidmode.setting.invulnerable_ender_dragon",
        "Invulnerable Ender Dragon",
    ),
    ("kidmode.setting.always_sunny", "Always sunny"),
    ("kidmode.setting.auto_sleep", "Auto sleep"),
    ("kidmode.setting.tame_for_loot", "Tame enemies for loot"),
    ("kidmode.tamed", "Tame %s"),
    ("kidmode.tamed.feminine", "Tame %s"),
    ("options.on", "ON"),
    ("options.off", "OFF"),
];

/// Polish. Terms follow docs/translations/dictionary.md.
pub const PL_PL: &[(&str, &str)] = &[
    ("kidmode.name", "Tryb dla dzieci"),
    (
        "kidmode.description",
        "Łagodniejszy świat: przyjazne i nieśmiertelne stworzenia, nieśmiertelny gracz, słoneczne dni i noce, które mijają same.",
    ),
    ("kidmode.setting.invulnerable_player", "Nieśmiertelny gracz"),
    ("kidmode.setting.friendly_enemies", "Przyjaźni wrogowie"),
    (
        "kidmode.setting.friendly_neutral_mobs",
        "Przyjazne neutralne stworzenia",
    ),
    (
        "kidmode.setting.friendly_mobs_fight_mobs",
        "Przyjazne stworzenia walczą z innymi",
    ),
    (
        "kidmode.setting.invulnerable_animals",
        "Nieśmiertelne zwierzęta",
    ),
    (
        "kidmode.setting.invulnerable_enemies",
        "Nieśmiertelni wrogowie",
    ),
    (
        "kidmode.setting.invulnerable_ender_dragon",
        "Nieśmiertelny Smok Endu",
    ),
    ("kidmode.setting.always_sunny", "Zawsze słonecznie"),
    ("kidmode.setting.auto_sleep", "Automatyczny sen"),
    ("kidmode.setting.tame_for_loot", "Oswajanie wrogów"),
    ("kidmode.tamed", "Oswojony %s"),
    ("kidmode.tamed.feminine", "Oswojona %s"),
    ("options.on", "Wł."),
    ("options.off", "Wył."),
];

/// The languages the mod ships, by the game's language code.
pub const LANGUAGES: &[(&str, &[(&str, &str)])] = &[("en_us", EN_US), ("pl_pl", PL_PL)];

/// The mod's own text for `key` in language `code` (a regional variant such
/// as `pl` reads its language's file), else English, else the key.
pub fn own_text(code: &str, key: &str) -> String {
    let code = code.to_ascii_lowercase();
    let find =
        |table: &[(&str, &'static str)]| table.iter().find(|(k, _)| *k == key).map(|(_, v)| *v);
    LANGUAGES
        .iter()
        .find(|(c, _)| *c == code || c.split('_').next() == Some(code.as_str()))
        .and_then(|(_, table)| find(table))
        .or_else(|| find(EN_US))
        .unwrap_or(key)
        .to_owned()
}

/// Text for `key` in language `code`: the game's current language first
/// (`game`: vanilla's keys, and resource packs that add
/// `assets/kidmode/lang/<code>.json`), then the mod's own.
pub fn text(code: &str, key: &str, game: impl Fn(&str) -> Option<String>) -> String {
    game(key).unwrap_or_else(|| own_text(code, key))
}

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

    /// The button's label, `Label: ON`, with `tr` translating a key.
    pub fn button(&self, tr: impl Fn(&str) -> String) -> String {
        let state = tr(if self.on() {
            "options.on"
        } else {
            "options.off"
        });
        format!("{}: {}", tr(&self.label_key()), state)
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

/// The settings page, in order: button `i` toggles `SETTINGS[i]`.
pub static SETTINGS: [&Setting; 10] = [
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

static LOADED: AtomicBool = AtomicBool::new(false);

/// Reads every setting from its file the first time, then keeps them. Shared by
/// the client thread (settings page) and the server thread (mobs), hence atomics.
pub fn load(get: impl Fn(&str) -> Option<String>) {
    if !LOADED.swap(true, Ordering::AcqRel) {
        for s in SETTINGS {
            s.set(s.read(get(s.key)));
        }
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
// Hooked into the mob engine (horizon_client_lib::mobs), not the mod events:
// `Mob::spares_players` is vanilla's `canAttack(player)`, which every target
// goal, revenge (`HurtByTargetGoal`), brain and kept target goes through. As a
// backup, what a friendly mob's own tick does to the player is dropped.

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

/// The mob whose tick is running on this thread, as `Mob::tick` started it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ticking {
    /// Where its events start in the tick's event list.
    pub from: usize,
    pub friendly: bool,
    pub spares_mobs: bool,
    pub invulnerable: bool,
}

thread_local! {
    /// The ticking mobs, innermost last, per thread: mobs tick on several threads.
    static TICKING: std::cell::RefCell<Vec<Ticking>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Mob `id` of type `kind` starts its tick with `len` events already queued.
pub fn tick_start(id: u32, kind: &str, len: usize) {
    let t = Ticking {
        from: len,
        friendly: friendly(id, kind),
        spares_mobs: spares_mobs_of(id, kind),
        invulnerable: invulnerable(kind),
    };
    TICKING.with(|s| s.borrow_mut().push(t));
}

/// The matching start, when the tick ends.
pub fn tick_end() -> Option<Ticking> {
    TICKING.with(|s| s.borrow_mut().pop())
}

/// The mob ticking on this thread, if any.
pub fn ticking() -> Option<Ticking> {
    TICKING.with(|s| s.borrow().last().copied())
}

// ---------------------------------------------------------------------------
// Invulnerable animals / Invulnerable enemies
// ---------------------------------------------------------------------------
// `Mob::refuses_damage` is vanilla's `isInvulnerableTo` at the start of
// `hurtServer`: a refused hit deals no damage, sets no attacker (no panic, no
// revenge) and gives no knockback. Fire panic and explosion pushes are hooked
// separately.

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

thread_local! {
    /// A witch's fire, hidden while it picks a potion (`witch::drink`).
    static HIDDEN_FIRE: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };
}

pub fn stash_fire(ticks: i32) {
    HIDDEN_FIRE.with(|f| f.set(ticks));
}

pub fn unstash_fire() -> i32 {
    HIDDEN_FIRE.with(|f| f.replace(0))
}

/// Damage types that still hurt an invulnerable mob or dragon.
pub const BYPASSES_INVULNERABILITY: &[&str] = &["minecraft:generic_kill", "minecraft:out_of_world"];

/// The ender dragon is hit by `damage_type` (none when not known): true to
/// refuse the hit.
pub fn refuse_dragon_damage(damage_type: Option<&str>) -> bool {
    INVULNERABLE_DRAGON.on() && !damage_type.is_some_and(|t| BYPASSES_INVULNERABILITY.contains(&t))
}

/// Invulnerable mobs that refused a hit, by id, for the explosion push (which
/// names the mob by id only).
static PROTECTED: Mutex<Option<HashMap<u32, bool>>> = Mutex::new(None);
const IDS_CAP: usize = 65_536;

/// Mob `entity` of type `kind` is hit (`bypasses`: `/kill`, the void): true to
/// refuse the hit.
pub fn refuse_damage(entity: u32, kind: &str, bypasses: bool) -> bool {
    if bypasses || !invulnerable(kind) {
        return false;
    }
    if let Ok(mut ids) = PROTECTED.lock() {
        let ids = ids.get_or_insert_with(HashMap::new);
        if ids.len() >= IDS_CAP {
            ids.clear();
        }
        ids.insert(entity, is_animal(kind));
    }
    true
}

/// Whether mob `entity`, seen refusing a hit, is still invulnerable.
pub fn protected_id(entity: u32) -> bool {
    let animal = PROTECTED
        .lock()
        .ok()
        .and_then(|ids| ids.as_ref()?.get(&entity).copied());
    match animal {
        Some(true) => INVULNERABLE_ANIMALS.on(),
        Some(false) => INVULNERABLE_ENEMIES.on(),
        None => false,
    }
}

// ---------------------------------------------------------------------------
// Tame enemies for loot (docs/taming.md)
// ---------------------------------------------------------------------------
// Feeding a mob its taming item may tame it (1 in 3). A tamed mob keeps the
// entity tag `kidmode_tamed` (saved with the world); the game copies the tagged
// mobs into `TAMED` before mobs tick, so the mob engine's hooks, which see a
// mob but not its tags, can ask `tamed(id)`.

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
    Tameable {
        mobs: &["cow"],
        loot: &["leather"],
        food: &["golden_carrot"],
    },
    Tameable {
        mobs: &["chicken"],
        loot: &["feather"],
        food: &["golden_carrot"],
    },
];

/// Mobs whose Polish name takes the feminine "Oswojona" (`kidmode.tamed.feminine`).
pub const FEMININE: &[&str] = &["witch", "magma_cube", "cow", "chicken"];

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
pub fn leashable(kind: &str) -> bool {
    TAME_FOR_LOOT.on() && tameable(kind).is_some()
}

/// The translation key and the mob-name key of a tamed mob's name.
pub fn tamed_name_keys(kind: &str) -> (&'static str, String) {
    let name = kind.strip_prefix("minecraft:").unwrap_or(kind);
    let key = if FEMININE.contains(&name) {
        "kidmode.tamed.feminine"
    } else {
        "kidmode.tamed"
    };
    (key, format!("entity.minecraft.{name}"))
}

/// A tamed mob's name: `format` (`Tame %s`) with the mob's name, its first
/// letter lower case ("Tame zombie", "Oswojony pająk").
pub fn tamed_name(format: &str, mob_name: &str) -> String {
    let mut chars = mob_name.chars();
    let lower = match chars.next() {
        Some(c) => c.to_lowercase().chain(chars).collect(),
        None => String::new(),
    };
    format.replacen("%s", &lower, 1)
}

/// The name a tamed mob of type `kind` gets, in language `code`; `game`
/// looks a key up in the game's current language (vanilla's mob names).
pub fn tamed_mob_name(kind: &str, code: &str, game: impl Fn(&str) -> Option<String>) -> String {
    let (format, mob_key) = tamed_name_keys(kind);
    let mob_name = game(&mob_key).unwrap_or_else(|| {
        kind.strip_prefix("minecraft:")
            .unwrap_or(kind)
            .replace('_', " ")
    });
    tamed_name(&text(code, format, &game), &mob_name)
}

/// Whether a lead on mob type `kind` hangs from the middle of its body (half
/// its height, on its centre) instead of vanilla's eye height. Cows and
/// chickens keep vanilla's.
pub fn lead_at_middle(kind: &str) -> bool {
    leashable(kind) && !matches!(kind, "minecraft:cow" | "minecraft:chicken")
}

/// Feeding results waiting for the mob's next tick to show their hearts
/// (`true`) or smoke: during a player's turn the game can't send particles.
static FEEDBACK: Mutex<Option<HashMap<u32, bool>>> = Mutex::new(None);

pub fn queue_feedback(id: u32, success: bool) {
    if let Ok(mut f) = FEEDBACK.lock() {
        let f = f.get_or_insert_with(HashMap::new);
        if f.len() >= IDS_CAP {
            f.clear();
        }
        f.insert(id, success);
    }
}

pub fn take_feedback(id: u32) -> Option<bool> {
    FEEDBACK.lock().ok()?.as_mut()?.remove(&id)
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

/// Mob `id` is tamed (`true`, it carries the tag) or not, as the game saw it
/// before mobs tick. `roll(n)` is a random number below `n`.
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
            if let Some(entry) = tameable(kind) {
                let i =
                    roll(entry.loot.len() as i32).clamp(0, entry.loot.len() as i32 - 1) as usize;
                out.loot = Some(format!("minecraft:{}", entry.loot[i]));
            }
        }
    }
    out
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

        assert!(refuse_damage(10, "minecraft:cow", false));
        assert!(protected_id(10));
        assert!(!refuse_damage(10, "minecraft:cow", true));
        assert!(!refuse_damage(11, "minecraft:zombie", false));
        INVULNERABLE_ENEMIES.set(true);
        assert!(refuse_damage(11, "minecraft:zombie", false));
        assert!(refuse_damage(13, "minecraft:zombified_piglin", false));
        assert!(refuse_damage(14, "minecraft:iron_golem", false));
        INVULNERABLE_ANIMALS.set(false);
        assert!(!refuse_damage(15, "minecraft:wolf", false));
        assert!(!refuse_damage(10, "minecraft:cow", false));
        assert!(!protected_id(10));
        assert!(protected_id(11));
        assert!(!refuse_damage(12, "minecraft:player", false));

        assert!(!refuse_dragon_damage(Some("minecraft:arrow")));
        INVULNERABLE_DRAGON.set(true);
        assert!(refuse_dragon_damage(Some("minecraft:arrow")));
        assert!(refuse_dragon_damage(None));
        assert!(!refuse_dragon_damage(Some("minecraft:generic_kill")));
    }

    #[test]
    fn tick_starts_nest() {
        tick_start(1, "minecraft:cow", 3);
        tick_start(2, "minecraft:pig", 5);
        assert_eq!(ticking().map(|t| t.from), Some(5));
        assert_eq!(tick_end().map(|t| t.from), Some(5));
        assert_eq!(tick_end().map(|t| t.from), Some(3));
        assert_eq!(tick_end(), None);
        assert_eq!(ticking(), None);
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

    /// `lang/<code>.json` as the mod writes it: the table in order, one key a line.
    fn lang_file(table: &[(&str, &str)]) -> String {
        let escape = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"");
        let lines: Vec<String> = table
            .iter()
            .map(|(k, v)| format!("  \"{}\": \"{}\"", escape(k), escape(v)))
            .collect();
        format!("{{\n{}\n}}\n", lines.join(",\n"))
    }

    #[test]
    fn translations_are_complete() {
        for (code, table) in LANGUAGES {
            let keys: Vec<&str> = table.iter().map(|(k, _)| *k).collect();
            let english: Vec<&str> = EN_US.iter().map(|(k, _)| *k).collect();
            assert_eq!(
                keys, english,
                "{code} has other keys than en_us, or in another order"
            );
            let path = format!(
                "{}/mixins/kidmode/lang/{code}.json",
                env!("CARGO_MANIFEST_DIR")
            );
            let file = std::fs::read_to_string(&path).unwrap_or_default();
            assert_eq!(
                file,
                lang_file(table),
                "{path} differs from {code} in runtime.rs"
            );
        }
        for s in SETTINGS {
            assert!(
                EN_US.iter().any(|(k, _)| *k == s.label_key()),
                "{} has no label",
                s.key
            );
        }
    }

    #[test]
    fn texts_follow_the_language() {
        let none = |_: &str| None;
        assert_eq!(own_text("pl_pl", "kidmode.name"), "Tryb dla dzieci");
        assert_eq!(own_text("PL_PL", "kidmode.name"), "Tryb dla dzieci");
        assert_eq!(own_text("de_de", "kidmode.name"), "Kid Mode");
        assert_eq!(own_text("pl_pl", "kidmode.nothing"), "kidmode.nothing");
        assert_eq!(text("pl_pl", "options.on", |_| Some("Wł.".into())), "Wł.");
        let tr = |k: &str| text("pl_pl", k, none);
        assert_eq!(
            INVULNERABLE_PLAYER.button(tr),
            format!(
                "Nieśmiertelny gracz: {}",
                tr(if INVULNERABLE_PLAYER.on() {
                    "options.on"
                } else {
                    "options.off"
                })
            )
        );
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
        assert_eq!(leftover("minecraft:milk_bucket"), Some("minecraft:bucket"));
        assert_eq!(leftover("minecraft:beef"), None);
        for t in TAMEABLE {
            assert!(!t.loot.is_empty() && !t.food.is_empty());
            for m in t.mobs {
                assert_eq!(
                    TAMEABLE.iter().filter(|o| o.mobs.contains(m)).count(),
                    1,
                    "{m} twice"
                );
            }
        }
        assert_eq!(
            tamed_name_keys("minecraft:witch").0,
            "kidmode.tamed.feminine"
        );
        assert_eq!(
            tamed_name_keys("minecraft:zombie"),
            ("kidmode.tamed", "entity.minecraft.zombie".into())
        );
        assert_eq!(tamed_name("Oswojony %s", "Pająk"), "Oswojony pająk");
        assert_eq!(tamed_name("Tame %s", "Zombie"), "Tame zombie");
        assert_eq!(
            tamed_mob_name("minecraft:cave_spider", "en_us", |_| None),
            "Tame cave spider"
        );
        let pl = |k: &str| (k == "entity.minecraft.witch").then(|| "Wiedźma".to_owned());
        assert_eq!(
            tamed_mob_name("minecraft:witch", "pl_pl", pl),
            "Oswojona wiedźma"
        );

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
        set_tamed(500, false, |_| 0);
        assert!(!tamed(500));
        assert_eq!(
            tamed_tick(500, "minecraft:creeper", true, |_| 0),
            TamedTick::default()
        );
        assert!(friendly(501, "minecraft:cow"));
        assert!(lead_at_middle("minecraft:zombie") && !lead_at_middle("minecraft:cow"));
        assert_eq!(take_feedback(7), None);
        queue_feedback(7, true);
        assert_eq!(take_feedback(7), Some(true));
        assert_eq!(take_feedback(7), None);
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
