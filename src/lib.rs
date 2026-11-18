//! Kid Mode (mod id `kidmode`), a native Horizon mod by veinmine: a gentler
//! world for children. The features and their settings are in docs/features.md.
//!
//! `logic` holds what the mod decides, free of the game (and its tests); this
//! file answers the game's hooks with it. Kid Mode uses the mod API's hooks
//! only: what no hook can do yet is listed in docs/missing.md.

pub mod logic;
pub mod tamed;

use logic as m;
use mod_api::{
    self, ClientHost, EndermanAngerEvent, EntityInvulnerabilityCheckEvent, ExplosionKnockbackEvent,
    ExplosionStartEvent, Flow, Hooks, Host, LevelSide, LivingChangeTargetEvent, Registrar,
    RegistrationContext, ServerHost, ServerTickEvent, SettingChangedEvent, SettingSpec,
    WeatherChangeEvent,
};

#[derive(Default)]
struct Mod;

/// How often (server ticks) the server reads the settings again.
const SETTINGS_EVERY: u64 = 20;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    /// The settings page (Options > Mods > Kid Mode > Config), in docs order.
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        for s in m::SETTINGS {
            if let Err(e) = r.register_setting(SettingSpec::toggle(s.key, s.label_key(), s.default))
            {
                eprintln!("[kidmode] setting {}: {e:?}", s.key);
            }
        }
    }

    /// The player changed a setting: it applies at once (in single player the
    /// server shares these settings).
    fn setting_changed(&self, host: &mut dyn ClientHost, event: &mut SettingChangedEvent<'_>) {
        if event.mod_id == m::ID {
            load_settings(&*host);
        }
    }

    /// Reads the settings once a second, so a dedicated server follows its own
    /// files.
    fn server_tick(&self, host: &mut dyn ServerHost, event: &mut ServerTickEvent<'_>) {
        if event.context.tick.get() % SETTINGS_EVERY == 0 {
            load_settings(&*host);
        }
    }

    /// Invulnerable player and invulnerable mobs: the hit is refused before it
    /// lands, as with vanilla's `Invulnerable` tag (no hurt flash, knockback,
    /// attacker or panic). `/kill` and the void still work.
    fn entity_invulnerability_check(
        &self,
        _: LevelSide,
        event: &mut EntityInvulnerabilityCheckEvent<'_>,
    ) {
        let bypasses = m::bypasses(event.source.damage_type);
        let refuse = if event.kind == "minecraft:player" {
            m::refuse_player_damage(bypasses)
        } else if event.kind == "minecraft:ender_dragon" {
            m::refuse_dragon_damage(Some(event.source.damage_type))
        } else {
            m::refuse_damage(event.kind, bypasses)
        };
        if refuse {
            event.invulnerable = true;
        }
    }

    /// Friendly mobs: a friendly mob never takes a player as its target, from a
    /// goal, a brain or revenge; unless friendly mobs may fight other mobs, it
    /// takes no target at all.
    fn living_change_target(
        &self,
        host: &dyn Host,
        event: &mut LivingChangeTargetEvent<'_>,
    ) -> Flow {
        let Some(target) = *event.new_target else {
            return Flow::Continue;
        };
        let mob = event.mob.entity();
        let Some(kind) = kind_of(host, mob) else {
            return Flow::Continue;
        };
        let is_player = kind_of(host, target.entity()).as_deref() == Some("minecraft:player");
        let drop = if is_player {
            m::friendly(mob.id(), &kind)
        } else {
            m::spares_mobs_of(mob.id(), &kind)
        };
        if drop {
            *event.new_target = None;
        }
        Flow::Continue
    }

    /// Friendly neutral mobs: an enderman doesn't mind being looked at.
    fn enderman_anger(&self, _: &dyn Host, event: &mut EndermanAngerEvent) -> Flow {
        if m::friendly_to_players("minecraft:enderman") {
            event.cancelled = true;
        }
        Flow::Continue
    }

    /// Always sunny: rain and thunder are refused the moment they would start.
    fn weather_change(&self, _: &mut dyn ServerHost, event: &mut WeatherChangeEvent<'_>) -> Flow {
        if m::ALWAYS_SUNNY.on() && (event.raining || event.thundering) {
            event.cancelled = true;
        }
        Flow::Continue
    }

    /// Explosions don't push invulnerable mobs.
    fn explosion_knockback(
        &self,
        host: &mut dyn ServerHost,
        event: &mut ExplosionKnockbackEvent<'_>,
    ) {
        let Some(id) = event.entity else { return };
        let Some(entity) = host.entity(id) else {
            return;
        };
        if kind_of(&*host, entity).is_some_and(|k| m::refuse_damage(&k, false)) {
            *event.knockback_velocity = [0.0; 3];
        }
    }

    /// An invulnerable creeper never explodes, even lit with flint and steel.
    fn explosion_start(
        &self,
        host: &mut dyn ServerHost,
        event: &mut ExplosionStartEvent<'_>,
    ) -> Flow {
        let creeper = event
            .explosion
            .source
            .and_then(|id| host.entity(id))
            .and_then(|e| kind_of(&*host, e))
            .is_some_and(|k| k == "minecraft:creeper");
        if creeper && m::invulnerable("minecraft:creeper") {
            event.cancelled = true;
        }
        Flow::Continue
    }
}

/// Reads every setting from the host (its files, through the settings API).
fn load_settings(host: &dyn Host) {
    m::load(|key| host.setting(m::ID, key));
}

/// An entity's type id (`minecraft:zombie`).
fn kind_of(host: &dyn Host, entity: mod_api::EntityRef) -> Option<String> {
    Some(entity.kind(host)?.id(host))
}
