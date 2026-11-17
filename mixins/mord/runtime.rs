// Mord, a native Horizon mod by veinmine. This file is both the mod's Cargo
// library (its tests: `cargo test`) and the module the bundle adds to the game
// as `crate::game::mord`. Module payloads cannot use inner attributes or `//!`
// comments, external `mod` files or `include!`. Features: docs/features.md.

use std::collections::HashMap;
use std::sync::Mutex;

use std::sync::atomic::{AtomicBool, Ordering};

/// The mod's id (its settings folder), name and version, as Options > Mods shows them.
pub const ID: &str = "mord";
pub const NAME: &str = "Mord";
pub const VERSION: &str = "0.4.0";
pub const DESCRIPTION: &str = "Friendly and invulnerable mobs. Features: docs/features.md.";

// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------

/// One on/off setting: its settings file (under
/// `horizon-mods/.horizon-mod-data/mord/`), its label and its default.
pub struct Setting {
    pub key: &'static str,
    pub label: &'static str,
    pub default: bool,
    value: AtomicBool,
}

impl Setting {
    const fn new(key: &'static str, label: &'static str, default: bool) -> Self {
        Setting { key, label, default, value: AtomicBool::new(default) }
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

    pub fn button(&self) -> String {
        format!("{}: {}", self.label, if self.on() { "ON" } else { "OFF" })
    }
}

pub fn stored(on: bool) -> &'static str {
    if on { "true" } else { "false" }
}

pub static FRIENDLY_ENEMIES: Setting = Setting::new("friendly_enemies", "Friendly enemies", true);
pub static FRIENDLY_NEUTRAL: Setting =
    Setting::new("friendly_neutral_mobs", "Friendly neutral mobs", true);
pub static FRIENDLY_FIGHT_MOBS: Setting =
    Setting::new("friendly_mobs_fight_mobs", "Friendly mobs fight other mobs", false);
pub static INVULNERABLE_ANIMALS: Setting =
    Setting::new("invulnerable_animals", "Invulnerable animals", true);
pub static INVULNERABLE_ENEMIES: Setting =
    Setting::new("invulnerable_enemies", "Invulnerable enemies", false);

/// The settings page, in order: button `i` toggles `SETTINGS[i]`.
pub static SETTINGS: [&Setting; 5] = [
    &FRIENDLY_ENEMIES,
    &FRIENDLY_NEUTRAL,
    &FRIENDLY_FIGHT_MOBS,
    &INVULNERABLE_ANIMALS,
    &INVULNERABLE_ENEMIES,
];

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
    "blaze", "bogged", "breeze", "cave_spider", "creaking", "creeper", "drowned",
    "elder_guardian", "ender_dragon", "endermite", "evoker", "ghast", "giant",
    "guardian", "hoglin", "husk", "illusioner", "magma_cube", "parched", "phantom",
    "piglin_brute", "pillager", "ravager", "shulker", "silverfish", "skeleton",
    "slime", "spider", "stray", "vex", "vindicator", "warden", "witch", "wither",
    "wither_skeleton", "zoglin", "zombie", "zombie_nautilus", "zombie_villager",
];

/// Vanilla's neutral mobs: they attack only when provoked or in some states.
pub const NEUTRAL: &[&str] = &[
    "bee", "dolphin", "enderman", "goat", "iron_golem", "llama", "panda", "piglin",
    "polar_bear", "trader_llama", "wolf", "zombified_piglin",
];

/// Animals: vanilla's passive and neutral creatures, land and water.
pub const ANIMALS: &[&str] = &[
    "armadillo", "axolotl", "bat", "bee", "camel", "cat", "chicken", "cod", "cow",
    "dolphin", "donkey", "fox", "frog", "glow_squid", "goat", "happy_ghast", "horse",
    "llama", "mooshroom", "mule", "nautilus", "ocelot", "panda", "parrot", "pig",
    "polar_bear", "pufferfish", "rabbit", "salmon", "sheep", "skeleton_horse",
    "sniffer", "squid", "strider", "tadpole", "trader_llama", "tropical_fish",
    "turtle", "wolf", "zombie_horse",
];

fn vanilla(kind: &str, list: &[&str]) -> bool {
    kind.strip_prefix("minecraft:").is_some_and(|name| list.contains(&name))
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

/// Harmful effects a mob can put on the player directly (not through a
/// projectile). Helpful ones (a dolphin's grace) are kept.
pub const HARMFUL_EFFECTS: &[&str] = &[
    "minecraft:blindness", "minecraft:darkness", "minecraft:hunger", "minecraft:levitation",
    "minecraft:mining_fatigue", "minecraft:nausea", "minecraft:poison", "minecraft:slowness",
    "minecraft:weakness", "minecraft:wither",
];

/// Whether a friendly mob of type `kind` may not attack other mobs either.
pub fn spares_mobs(kind: &str) -> bool {
    !FRIENDLY_FIGHT_MOBS.on() && friendly_to_players(kind)
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

/// Mob type `kind` starts its tick with `len` events already queued.
pub fn tick_start(kind: &str, len: usize) {
    let t = Ticking {
        from: len,
        friendly: friendly_to_players(kind),
        spares_mobs: spares_mobs(kind),
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
    let animal = PROTECTED.lock().ok().and_then(|ids| ids.as_ref()?.get(&entity).copied());
    match animal {
        Some(true) => INVULNERABLE_ANIMALS.on(),
        Some(false) => INVULNERABLE_ENEMIES.on(),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The settings are global, so the tests that change them run in one test.
    #[test]
    fn behaviour_follows_the_settings() {
        assert!(FRIENDLY_ENEMIES.default && FRIENDLY_NEUTRAL.default && !FRIENDLY_FIGHT_MOBS.default);
        assert!(INVULNERABLE_ANIMALS.default && !INVULNERABLE_ENEMIES.default);

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
    }

    #[test]
    fn tick_starts_nest() {
        tick_start("minecraft:cow", 3);
        tick_start("minecraft:pig", 5);
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

    #[test]
    fn groups() {
        assert!(is_hostile("minecraft:zombie") && !is_hostile("minecraft:wolf"));
        assert!(is_neutral("minecraft:enderman") && !is_neutral("minecraft:zombie"));
        assert!(is_animal("minecraft:cow") && is_animal("minecraft:wolf"));
        assert!(!is_animal("minecraft:iron_golem") && !is_animal("othermod:cow"));
        for name in HOSTILE {
            assert!(!NEUTRAL.contains(name), "{name} is both hostile and neutral");
        }
    }
}
