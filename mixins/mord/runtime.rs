// Mord, a native Horizon mod by veinmine. This file is both the mod's Cargo
// library (its tests: `cargo test`) and the module the bundle adds to the game
// as `crate::game::mord`. Module payloads cannot use inner attributes or `//!`
// comments, external `mod` files or `include!`. Features: docs/features.md.

use std::collections::HashSet;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

/// The mod's id (its settings folder), name and version, as Options > Mods shows them.
pub const ID: &str = "mord";
pub const NAME: &str = "Mord";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "Friendly enemies: hostile mobs never attack you.";

// ---------------------------------------------------------------------------
// Setting: Friendly enemies
// ---------------------------------------------------------------------------

/// The settings file (under `horizon-mods/.horizon-mod-data/mord/`) that keeps it.
pub const FRIENDLY_KEY: &str = "friendly_enemies";

/// On unless the file says `false`: the feature is enabled by default.
pub fn read_friendly(stored: Option<String>) -> bool {
    !matches!(stored.as_deref().map(str::trim), Some("false"))
}

pub fn stored_friendly(on: bool) -> &'static str {
    if on { "true" } else { "false" }
}

pub fn friendly_label(on: bool) -> String {
    format!("Friendly enemies: {}", if on { "ON" } else { "OFF" })
}

// Shared by the client thread (settings page) and the server thread (mob AI),
// so atomics rather than thread locals.
static FRIENDLY: AtomicBool = AtomicBool::new(true);
static FRIENDLY_LOADED: AtomicBool = AtomicBool::new(false);

/// Reads the setting from its file the first time it is asked, then keeps it.
pub fn load_friendly(get: impl FnOnce() -> Option<String>) -> bool {
    if !FRIENDLY_LOADED.swap(true, Ordering::AcqRel) {
        FRIENDLY.store(read_friendly(get()), Ordering::Release);
    }
    FRIENDLY.load(Ordering::Acquire)
}

/// The setting as last loaded or set (on until loaded).
pub fn friendly() -> bool {
    FRIENDLY.load(Ordering::Acquire)
}

/// The settings page changed it.
pub fn set_friendly(on: bool) {
    FRIENDLY_LOADED.store(true, Ordering::Release);
    FRIENDLY.store(on, Ordering::Release);
}

// ---------------------------------------------------------------------------
// Feature: hostile mobs never attack the player
// ---------------------------------------------------------------------------

/// Vanilla's hostile mobs, including the ones that only turn hostile when
/// provoked (enderman, piglins, spiders by day). Tamable and utility mobs
/// (wolves, iron golems, bees, llamas, polar bears) are not enemies and keep
/// their behaviour.
pub const HOSTILE: &[&str] = &[
    "blaze", "bogged", "breeze", "cave_spider", "creaking", "creeper", "drowned",
    "elder_guardian", "ender_dragon", "enderman", "endermite", "evoker", "ghast",
    "giant", "guardian", "hoglin", "husk", "illusioner", "magma_cube", "phantom",
    "piglin", "piglin_brute", "pillager", "ravager", "shulker", "silverfish",
    "skeleton", "slime", "spider", "stray", "vex", "vindicator", "warden", "witch",
    "wither", "wither_skeleton", "zoglin", "zombie", "zombie_villager",
    "zombified_piglin",
];

/// Whether entity type `kind` (`minecraft:zombie`) is a hostile mob.
pub fn is_hostile(kind: &str) -> bool {
    match kind.strip_prefix("minecraft:") {
        Some(name) => HOSTILE.contains(&name),
        None => false,
    }
}

/// Mobs seen trying to target a player, so a hit from them can be recognised
/// (the damage hook names the attacker by id only).
static HOSTILE_IDS: Mutex<Option<HashSet<u32>>> = Mutex::new(None);
const HOSTILE_IDS_CAP: usize = 65_536;

fn remember_hostile(entity: u32) {
    if let Ok(mut ids) = HOSTILE_IDS.lock() {
        let ids = ids.get_or_insert_with(HashSet::new);
        if ids.len() >= HOSTILE_IDS_CAP {
            ids.clear();
        }
        ids.insert(entity);
    }
}

fn known_hostile(entity: u32) -> bool {
    HOSTILE_IDS
        .lock()
        .map(|ids| ids.as_ref().is_some_and(|ids| ids.contains(&entity)))
        .unwrap_or(false)
}

/// Mob `entity` of type `kind` is about to target `player` (a UUID, none for
/// a mob or letting go): true to refuse it. This also covers revenge after
/// the player hit it first, which goes through the same targeting.
pub fn refuse_target(on: bool, entity: u32, kind: &str, player: Option<u128>) -> bool {
    if player.is_none() || !is_hostile(kind) {
        return false;
    }
    remember_hostile(entity);
    on
}

/// A hit on a player (`victim_player`) by `attacker` is about to land: true
/// to cancel it when the attacker is a hostile mob. Catches a mob that held
/// its target from before the setting was turned on.
pub fn refuse_hit(
    on: bool,
    victim_player: Option<u128>,
    attacker: Option<u32>,
    attacker_player: Option<u128>,
) -> bool {
    on && victim_player.is_some()
        && attacker_player.is_none()
        && attacker.is_some_and(known_hostile)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_lowercase_ascii() {
        assert!(ID.bytes().all(|b| b.is_ascii_lowercase() || b == b'_'));
    }

    #[test]
    fn friendly_enemies_is_on_by_default() {
        assert!(read_friendly(None));
        assert!(read_friendly(Some("true".into())));
        assert!(read_friendly(Some("garbage".into())));
        assert!(!read_friendly(Some("false\n".into())));
        assert_eq!(friendly_label(true), "Friendly enemies: ON");
        assert_eq!(friendly_label(false), "Friendly enemies: OFF");
    }

    #[test]
    fn hostile_mobs_are_recognised() {
        assert!(is_hostile("minecraft:zombie"));
        assert!(is_hostile("minecraft:creeper"));
        assert!(!is_hostile("minecraft:wolf"));
        assert!(!is_hostile("minecraft:iron_golem"));
        assert!(!is_hostile("othermod:zombie"));
    }

    #[test]
    fn hostile_mobs_never_target_players_when_on() {
        assert!(refuse_target(true, 1, "minecraft:skeleton", Some(42)));
        assert!(!refuse_target(false, 2, "minecraft:skeleton", Some(42)));
        assert!(!refuse_target(true, 3, "minecraft:skeleton", None));
        assert!(!refuse_target(true, 4, "minecraft:wolf", Some(42)));
    }

    #[test]
    fn hits_from_known_hostiles_on_players_are_cancelled() {
        refuse_target(true, 100, "minecraft:zombie", Some(7));
        assert!(refuse_hit(true, Some(7), Some(100), None));
        assert!(!refuse_hit(false, Some(7), Some(100), None));
        assert!(!refuse_hit(true, None, Some(100), None));
        assert!(!refuse_hit(true, Some(7), Some(999), None));
        assert!(!refuse_hit(true, Some(7), Some(100), Some(8)));
    }
}
