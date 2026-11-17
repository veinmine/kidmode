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
pub const VERSION: &str = "0.2.0";
pub const DESCRIPTION: &str = "Friendly and invulnerable mobs.";

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
pub static INVULNERABLE_ANIMALS: Setting =
    Setting::new("invulnerable_animals", "Invulnerable animals", true);
pub static INVULNERABLE_ENEMIES: Setting =
    Setting::new("invulnerable_enemies", "Invulnerable enemies", false);

/// The settings page, in order: button `i` toggles `SETTINGS[i]`.
pub static SETTINGS: [&Setting; 4] =
    [&FRIENDLY_ENEMIES, &FRIENDLY_NEUTRAL, &INVULNERABLE_ANIMALS, &INVULNERABLE_ENEMIES];

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Attitude {
    Hostile,
    Neutral,
}

impl Attitude {
    fn of(kind: &str) -> Option<Self> {
        if is_hostile(kind) {
            Some(Attitude::Hostile)
        } else if is_neutral(kind) {
            Some(Attitude::Neutral)
        } else {
            None
        }
    }

    fn friendly(self) -> bool {
        match self {
            Attitude::Hostile => FRIENDLY_ENEMIES.on(),
            Attitude::Neutral => FRIENDLY_NEUTRAL.on(),
        }
    }
}

/// Mobs seen trying to target a player, by id, so a hit from them can be
/// recognised (the damage hook names the attacker by id only).
static ATTACKERS: Mutex<Option<HashMap<u32, Attitude>>> = Mutex::new(None);
const IDS_CAP: usize = 65_536;

fn remember_attacker(entity: u32, attitude: Attitude) {
    if let Ok(mut ids) = ATTACKERS.lock() {
        let ids = ids.get_or_insert_with(HashMap::new);
        if ids.len() >= IDS_CAP {
            ids.clear();
        }
        ids.insert(entity, attitude);
    }
}

fn attacker_attitude(entity: u32) -> Option<Attitude> {
    ATTACKERS.lock().ok()?.as_ref()?.get(&entity).copied()
}

/// Mob `entity` of type `kind` is about to target `player` (a UUID, none for
/// a mob or letting go): true to refuse it. Revenge after the player hit it
/// first goes through the same targeting, so it is refused too.
pub fn refuse_target(entity: u32, kind: &str, player: Option<u128>) -> bool {
    if player.is_none() {
        return false;
    }
    let Some(attitude) = Attitude::of(kind) else { return false };
    remember_attacker(entity, attitude);
    attitude.friendly()
}

/// A hit on a player (`victim_player`) by `attacker` is about to land: true to
/// cancel it when the attacker is a friendly mob. Catches a mob that held its
/// target from before the setting was turned on.
pub fn refuse_hit_on_player(
    victim_player: Option<u128>,
    attacker: Option<u32>,
    attacker_player: Option<u128>,
) -> bool {
    victim_player.is_some()
        && attacker_player.is_none()
        && attacker.and_then(attacker_attitude).is_some_and(Attitude::friendly)
}

// ---------------------------------------------------------------------------
// Invulnerable animals / Invulnerable enemies
// ---------------------------------------------------------------------------

/// Damage that still applies, so `/kill` and falling out of the world work.
pub const ALWAYS_HURTS: &[&str] = &["minecraft:generic_kill", "minecraft:out_of_world"];

/// Whether mob type `kind` is invulnerable with the current settings.
pub fn invulnerable(kind: &str) -> bool {
    (INVULNERABLE_ANIMALS.on() && is_animal(kind))
        || (INVULNERABLE_ENEMIES.on() && is_hostile(kind))
}

/// Invulnerable mobs seen being hit, by id, for the explosion push (which
/// names the mob by id only).
static PROTECTED: Mutex<Option<HashMap<u32, &'static str>>> = Mutex::new(None);

fn remember_protected(entity: u32, kind: &str) {
    let kind = if is_animal(kind) { "animal" } else { "enemy" };
    if let Ok(mut ids) = PROTECTED.lock() {
        let ids = ids.get_or_insert_with(HashMap::new);
        if ids.len() >= IDS_CAP {
            ids.clear();
        }
        ids.insert(entity, kind);
    }
}

/// Whether mob `entity`, seen hit before, is still invulnerable.
pub fn protected_id(entity: u32) -> bool {
    let group = PROTECTED.lock().ok().and_then(|ids| ids.as_ref()?.get(&entity).copied());
    match group {
        Some("animal") => INVULNERABLE_ANIMALS.on(),
        Some(_) => INVULNERABLE_ENEMIES.on(),
        None => false,
    }
}

/// `victim` of type `kind` is about to take `damage_type`: true to cancel the
/// hit entirely. A canceled hit deals no damage, no knockback, no hurt flash and
/// leaves no attacker to panic from or take revenge on.
pub fn refuse_damage(victim: u32, kind: &str, damage_type: &str) -> bool {
    if !invulnerable(kind) || ALWAYS_HURTS.contains(&damage_type) {
        return false;
    }
    remember_protected(victim, kind);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // The settings are global, so the tests that change them run in one test.
    #[test]
    fn behaviour_follows_the_settings() {
        // Defaults.
        assert!(FRIENDLY_ENEMIES.default && FRIENDLY_NEUTRAL.default);
        assert!(INVULNERABLE_ANIMALS.default && !INVULNERABLE_ENEMIES.default);

        // Friendly: hostile and neutral mobs may not target players.
        assert!(refuse_target(1, "minecraft:skeleton", Some(42)));
        assert!(refuse_target(2, "minecraft:wolf", Some(42)));
        assert!(!refuse_target(3, "minecraft:skeleton", None));
        assert!(!refuse_target(4, "minecraft:cow", Some(42)));
        assert!(refuse_hit_on_player(Some(7), Some(1), None));
        assert!(!refuse_hit_on_player(Some(7), Some(999), None));
        assert!(!refuse_hit_on_player(None, Some(1), None));
        assert!(!refuse_hit_on_player(Some(7), Some(1), Some(8)));
        FRIENDLY_NEUTRAL.set(false);
        assert!(!refuse_target(2, "minecraft:wolf", Some(42)));
        assert!(!refuse_hit_on_player(Some(7), Some(2), None));
        assert!(refuse_target(1, "minecraft:skeleton", Some(42)));
        FRIENDLY_ENEMIES.set(false);
        assert!(!refuse_target(1, "minecraft:skeleton", Some(42)));
        assert!(!refuse_hit_on_player(Some(7), Some(1), None));

        // Invulnerable: animals by default, enemies when turned on.
        assert!(refuse_damage(10, "minecraft:cow", "minecraft:lava"));
        assert!(protected_id(10));
        assert!(!refuse_damage(10, "minecraft:cow", "minecraft:generic_kill"));
        assert!(!refuse_damage(11, "minecraft:zombie", "minecraft:player_attack"));
        INVULNERABLE_ENEMIES.set(true);
        assert!(refuse_damage(11, "minecraft:zombie", "minecraft:on_fire"));
        INVULNERABLE_ANIMALS.set(false);
        assert!(!refuse_damage(10, "minecraft:cow", "minecraft:lava"));
        assert!(!protected_id(10));
        assert!(protected_id(11));
        assert!(!refuse_damage(12, "minecraft:player", "minecraft:lava"));
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
