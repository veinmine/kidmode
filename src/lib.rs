//! Kid Mode (mod id `kidmode`), a native Horizon mod by veinmine: a gentler
//! world for children. The features and their settings are in docs/features.md.
//!
//! `logic` holds what the mod decides, free of the game (and its tests); this
//! file answers the game's server hooks with it, and `hint` draws the taming
//! hints on the client. Kid Mode uses the mod API's hooks only, no mixins:
//! what no hook can do yet is listed in docs/missing.md.

pub mod hint;
pub mod logic;
pub mod tamed;

use std::sync::OnceLock;

use logic as m;
use mod_api::{
    self, ClientHost, CreeperIgniteEvent, Despawn, EnderEntityTeleportEvent, EndermanAngerEvent,
    EntityInteractEvent, EntityInvulnerabilityCheckEvent, EntityRef, EntityTickListenedEvent,
    EntityTickPostEvent, ExplosionKnockbackEvent, ExplosionStartEvent, Flow, GameRuleValue, Hand,
    Holder, Hooks, Host, HostSide, Identifier, InteractionResult, ItemStack, LevelSide,
    LivingChangeTargetEvent, LivingEntityUseItemStartEvent, LivingKnockbackEvent, LoopContext,
    MobDespawnEvent, ParticleKey, ParticleOptions, Registrar, RegistrationContext,
    RenderLivingPostEvent, ServerHost, ServerTickEvent, SettingChangedEvent, SettingSpec, StatType,
    Tag, WeatherChangeEvent,
};

#[derive(Default)]
struct Mod;

/// How often (server ticks) the server reads the settings again.
const SETTINGS_EVERY: u64 = 20;

/// The particles the mod shows, requested at registration.
struct Particles {
    heart: ParticleKey,
    smoke: ParticleKey,
    sparkle: ParticleKey,
}

static PARTICLES: OnceLock<Particles> = OnceLock::new();

#[mod_api::horizon_mod]
impl Hooks for Mod {
    /// The settings page (Options > Mods > Kid Mode > Config), in docs order,
    /// and the particles taming shows.
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        for s in m::SETTINGS {
            if let Err(e) = r.register_setting(SettingSpec::toggle(s.key, s.label_key(), s.default))
            {
                eprintln!("[kidmode] setting {}: {e:?}", s.key);
            }
        }
        let _ = PARTICLES.set(Particles {
            heart: r.particle_key("minecraft:heart"),
            smoke: r.particle_key("minecraft:smoke"),
            sparkle: r.particle_key("minecraft:happy_villager"),
        });
    }

    /// The player changed a setting: it applies at once (in single player the
    /// server shares these settings).
    fn setting_changed(&self, host: &mut dyn ClientHost, event: &mut SettingChangedEvent<'_>) {
        if event.mod_id == m::ID {
            load_settings(&*host);
        }
    }

    /// Reads the settings once a second, so a dedicated server follows its own
    /// files; Auto sleep.
    fn server_tick(&self, host: &mut dyn ServerHost, event: &mut ServerTickEvent<'_>) {
        if event.context.tick.get().is_multiple_of(SETTINGS_EVERY) {
            load_settings(&*host);
        }
        if matches!(
            event.phase,
            mod_api::ServerTickPhase::ServerTickEnd {
                world_running: true
            }
        ) {
            auto_sleep(host);
        }
    }

    /// Invulnerable player and invulnerable mobs: the hit is refused before it
    /// lands, as with vanilla's `Invulnerable` tag (no hurt flash, knockback,
    /// attacker or panic). `/kill` and the void still work. Friendly mobs: a
    /// hit on a player from a friendly mob, its projectile or its explosion
    /// never lands either (the Ender Dragon's breath and wings, the Wither's
    /// skulls, a goat's ram, the Warden's sonic boom).
    fn entity_invulnerability_check(
        &self,
        _: LevelSide,
        event: &mut EntityInvulnerabilityCheckEvent<'_>,
    ) {
        let source = event.source;
        let bypasses = m::bypasses(source.damage_type);
        let refuse = if event.kind == "minecraft:player" {
            m::refuse_player_damage(bypasses)
                || (!bypasses && m::hit_by_friendly(source.causing, source.direct))
        } else if event.kind == "minecraft:ender_dragon" {
            m::refuse_dragon_damage(Some(source.damage_type))
        } else {
            m::refuse_damage(event.kind, bypasses)
        };
        if refuse {
            event.invulnerable = true;
        }
    }

    /// A friendly mob's push on a player (a goat's ram, a ravager's roar) is
    /// dropped with its hit.
    fn living_knockback(
        &self,
        _: &mut dyn mod_api::LevelRandomHost,
        event: &mut LivingKnockbackEvent<'_>,
    ) -> Flow {
        if event.kind == "minecraft:player" && m::hit_by_friendly(event.attacker, 0) {
            return Flow::Cancel;
        }
        Flow::Continue
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
        let drop = if is_player(host, target.entity()) {
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
        if m::friendly(event.enderman.id(), "minecraft:enderman") {
            return Flow::Cancel;
        }
        Flow::Continue
    }

    /// Invulnerable enemies: a witch doesn't drink fire resistance.
    fn living_entity_use_item_start(
        &self,
        host: HostSide,
        event: &mut LivingEntityUseItemStartEvent<'_>,
    ) -> Flow {
        let HostSide::Server(host) = host else {
            return Flow::Continue;
        };
        let witch = event
            .entity
            .and_then(|id| u32::try_from(id).ok())
            .and_then(|id| host.entity(id))
            .and_then(|e| kind_of(&*host, e))
            .is_some_and(|k| k == "minecraft:witch");
        if witch && m::invulnerable("minecraft:witch") {
            return Flow::Cancel;
        }
        Flow::Continue
    }

    /// Always sunny: rain and thunder are refused the moment they would start.
    fn weather_change(&self, _: &mut dyn ServerHost, event: &mut WeatherChangeEvent<'_>) -> Flow {
        if m::ALWAYS_SUNNY.on() && (event.raining || event.thundering) {
            return Flow::Cancel;
        }
        Flow::Continue
    }

    /// Explosions don't push invulnerable mobs.
    fn explosion_knockback(
        &self,
        host: &mut dyn ServerHost,
        event: &mut ExplosionKnockbackEvent<'_>,
    ) {
        let Some(entity) = event.entity.and_then(|id| host.entity(id)) else {
            return;
        };
        if kind_of(&*host, entity).is_some_and(|k| m::refuse_damage(&k, false)) {
            *event.knockback_velocity = [0.0; 3];
        }
    }

    /// An invulnerable creeper never explodes, even lit with flint and steel. A
    /// friendly mob that may not fight sets off no explosion (the Wither's
    /// spawn blast and skulls).
    fn explosion_start(
        &self,
        host: &mut dyn ServerHost,
        event: &mut ExplosionStartEvent<'_>,
    ) -> Flow {
        let explosion = event.explosion;
        let mut cancel = false;
        for id in [explosion.source, explosion.owner].into_iter().flatten() {
            let Some(kind) = host.entity(id).and_then(|e| kind_of(&*host, e)) else {
                continue;
            };
            cancel |= kind == "minecraft:creeper" && m::invulnerable(&kind);
            cancel |= m::spares_mobs_of(id, &kind);
        }
        if cancel {
            return Flow::Cancel;
        }
        Flow::Continue
    }

    /// A creeper lit with flint and steel: an invulnerable one stays unlit.
    fn creeper_ignite(&self, host: &mut dyn ServerHost, event: &mut CreeperIgniteEvent<'_>) {
        if m::invulnerable("minecraft:creeper")
            && let Some(creeper) = host.entity(event.entity)
        {
            let _ = creeper.merge_nbt(host, [("ignited".into(), Tag::Byte(0))].into());
        }
    }

    /// Mob ticks are told only to a mod that asks for them.
    fn entity_tick_listened(&self, _: &mut dyn ServerHost, event: &mut EntityTickListenedEvent) {
        event.listened = true;
    }

    /// After a mob's tick: note its type (for the damage hooks), make sure a
    /// friendly mob has no player target, and run a tamed mob's loot and
    /// sparkles.
    fn entity_tick_post(&self, host: &mut dyn ServerHost, event: &mut EntityTickPostEvent<'_>) {
        let (id, kind) = (event.entity, event.kind);
        let Some(entity) = host.entity(id) else {
            return;
        };
        let Some(mob) = host.mob(entity) else {
            return;
        };
        m::note_mob(id, kind);
        if m::friendly(id, kind)
            && let Some(target) = mob.target(&*host)
            && (is_player(&*host, target.entity()) || m::spares_mobs_of(id, kind))
        {
            mob.set_target(host, None);
        }
        tamed_tick(host, entity, kind);
    }

    /// A tamed mob never despawns.
    fn mob_despawn(&self, _: &dyn Host, event: &mut MobDespawnEvent<'_>) {
        if m::tamed(event.mob.entity().id()) {
            *event.result = Despawn::Deny;
        }
    }

    /// A tamed enderman or shulker never teleports away (and breaks its lead).
    fn ender_entity_teleport(
        &self,
        _: &dyn LoopContext,
        event: &mut EnderEntityTeleportEvent<'_>,
    ) -> Flow {
        if m::tamed(event.entity) {
            return Flow::Cancel;
        }
        Flow::Continue
    }

    /// Right click on a mob: its taming item may tame it, and a lead leads
    /// every enemy and neutral mob that can be tamed.
    fn entity_interact(
        &self,
        host: HostSide,
        event: &mut EntityInteractEvent<'_>,
    ) -> Option<InteractionResult> {
        let HostSide::Server(host) = host else {
            return None;
        };
        let stack = event.stack?;
        let item = stack.item().id(&*host);
        let entity = host.entity(u32::try_from(event.entity).ok()?)?;
        let player = host.players().acting_player()?;
        if item == "minecraft:lead" && m::leashable(event.kind) {
            return lead(host, entity, player, event.hand);
        }
        if !m::tames(event.kind, &item) || is_tamed(&mut *host, entity) {
            return None;
        }
        feed(host, entity, player, event.hand, &item);
        Some(InteractionResult::Success)
    }

    /// The taming hint over the mob the player looks at.
    fn render_living_post(&self, host: &dyn ClientHost, event: &mut RenderLivingPostEvent<'_>) {
        hint::render(host, event);
    }
}

/// Reads every setting from the host (its files, through the settings API).
fn load_settings(host: &dyn Host) {
    m::load(|key| host.setting(m::ID, key));
}

/// An entity's type id (`minecraft:zombie`).
fn kind_of(host: &dyn Host, entity: EntityRef) -> Option<String> {
    Some(entity.kind(host)?.id(host))
}

fn is_player(host: &dyn Host, entity: EntityRef) -> bool {
    kind_of(host, entity).as_deref() == Some("minecraft:player")
}

/// Whether mob `entity` is tamed as Kid Mode counts it: it carries the
/// `kidmode:tamed` attachment or the `kidmode_tamed` tag (`/tag`).
fn is_tamed(host: &mut dyn Host, entity: EntityRef) -> bool {
    let attached = tamed::TAMED
        .has(&mut *host, Holder::Entity(entity.id()))
        .unwrap_or(false);
    attached
        || entity
            .tags(&*host)
            .is_some_and(|tags| tags.iter().any(|t| t == m::TAMED_TAG))
}

/// Auto sleep: once the day has ended in the Overworld, skip to the morning
/// as when everyone sleeps (the night counts as rest, the rain clears).
fn auto_sleep(host: &mut dyn ServerHost) {
    if !m::AUTO_SLEEP.on() {
        return;
    }
    let Some(level) = host.level("minecraft:overworld") else {
        return;
    };
    let Some(time) = level.clock_time(&*host) else {
        return;
    };
    let advance = level.game_rule(&*host, "minecraft:advance_time");
    if !matches!(advance, Ok(Some(GameRuleValue::Bool(true)))) || !m::auto_sleep_now(time) {
        return;
    }
    level.set_clock_time(host, m::next_morning(time));
    let weather = level.game_rule(&*host, "minecraft:advance_weather");
    if matches!(weather, Ok(Some(GameRuleValue::Bool(true)))) {
        level.set_weather(host, 0, 0, false, false);
    }
    for player in level.online_players(&*host).unwrap_or_default() {
        player.reset_stat(host, StatType::Custom, "minecraft:time_since_rest");
    }
}

/// A tamed mob's tick: it stays tamed (the setting on), drops its loot now
/// and then and shows its sparkles. Vanilla's pets count when tamed the
/// vanilla way.
fn tamed_tick(host: &mut dyn ServerHost, entity: EntityRef, kind: &str) {
    let id = entity.id();
    let pet = m::pet_loot(kind).is_some();
    if m::tameable(kind).is_none() && !pet {
        return;
    }
    let tamed = if pet {
        host.living(entity)
            .and_then(|l| l.is_tamed(&*host))
            .unwrap_or(false)
    } else {
        is_tamed(&mut *host, entity)
    };
    m::set_tamed(id, tamed, m::roll);
    if !tamed {
        return;
    }
    if let Some(mob) = host.mob(entity) {
        mob.set_persistence_required(host);
    }
    // One tamed with `/tag` gets the attachment, so players' games know it.
    if !pet
        && !tamed::TAMED
            .has(&mut *host, Holder::Entity(id))
            .unwrap_or(true)
    {
        let _ = tamed::TAMED.set(host, Holder::Entity(id), tamed::Tamed::default());
    }
    let adult = !host
        .living(entity)
        .and_then(|l| l.baby(&*host))
        .unwrap_or(false);
    let tick = m::tamed_tick(id, kind, adult, m::roll);
    if let Some(item) = tick.loot {
        let item = if kind == "minecraft:frog" {
            m::frog_loot(&frog_variant(host, entity)).to_owned()
        } else {
            item
        };
        if let Some(stack) = stack_of(&*host, &item) {
            let _ = entity.spawn_at_location(host, stack, 0.0);
        }
    }
    if tick.sparkle
        && let Some(p) = PARTICLES.get()
    {
        particles(host, entity, p.sparkle, 3, 0.0, true);
    }
}

/// A frog's colour (`minecraft:warm`), from its saved data.
fn frog_variant(host: &mut dyn ServerHost, entity: EntityRef) -> String {
    match entity
        .nbt(host)
        .and_then(|n| n.get(&"variant".into()).cloned())
    {
        Some(Tag::String(v)) => v.to_string_lossy(),
        _ => String::new(),
    }
}

/// One item of `id` (`minecraft:bone`).
fn stack_of(host: &dyn Host, id: &str) -> Option<ItemStack> {
    let key = host.item_key(&Identifier::parse_namespaced(id)?)?;
    Some(ItemStack::of(key))
}

/// Particles on mob `entity`: around its middle (hearts, smoke), or over its
/// head (`above`, the sparkles), as vanilla's taming particles.
fn particles(
    host: &mut dyn ServerHost,
    entity: EntityRef,
    key: ParticleKey,
    count: i32,
    speed: f64,
    above: bool,
) {
    let (Some(level), Some([lo, hi])) = (entity.level(&*host), entity.bounding_box(&*host)) else {
        return;
    };
    let mid = |i: usize| (lo[i] + hi[i]) / 2.0;
    let half = |i: usize| (hi[i] - lo[i]) / 2.0;
    let (at, spread) = if above {
        (
            [mid(0), hi[1] + 0.3, mid(2)],
            [half(0) / 1.5, 0.2, half(2) / 1.5],
        )
    } else {
        (
            [mid(0), mid(1) + 0.3, mid(2)],
            [half(0), (hi[1] - lo[1]) / 3.0, half(2)],
        )
    };
    let options = ParticleOptions::simple(key);
    let _ = level.send_particles(host, &options, None, at, spread, speed, count);
}

/// Feeding mob `entity` its taming item: 1 in 3 tames it (hearts), else smoke.
/// The item is used up, except in creative; a milk bucket leaves a bucket.
fn feed(
    host: &mut dyn ServerHost,
    entity: EntityRef,
    player: mod_api::PlayerRef,
    hand: Hand,
    item: &str,
) {
    let success = m::roll(3) == 0;
    if success {
        let tamer = tamed::Tamed {
            owner: Some(player.uuid()),
            owner_name: player.name(&*host),
            profile: player.profile(&*host).map(|p| tamed::profile_tag(&p)),
        };
        let _ = tamed::TAMED.set(host, Holder::Entity(entity.id()), tamer);
        m::set_tamed(entity.id(), true, m::roll);
        if let Some(mob) = host.mob(entity) {
            mob.set_persistence_required(host);
            mob.set_target(host, None);
        }
    }
    if let Some(p) = PARTICLES.get() {
        let key = if success { p.heart } else { p.smoke };
        particles(host, entity, key, 7, 0.02, false);
    }
    if !player.is_creative(&*host).unwrap_or(false) {
        use_up(host, player, hand, m::leftover(item));
    }
}

/// Uses up one item from the player's `hand`, leaving `rest` (a bucket) in
/// its place, or in the inventory when more remain.
fn use_up(host: &mut dyn ServerHost, player: mod_api::PlayerRef, hand: Hand, rest: Option<&str>) {
    let Some(living) = host.living(player.entity()) else {
        return;
    };
    let held = living.hand_item(&*host, hand);
    let left = held.and_then(|s| s.shrink(1));
    let rest = rest.and_then(|r| stack_of(&*host, r));
    match (left, rest) {
        (None, rest) => {
            let _ = living.set_hand_item(host, hand, rest);
        }
        (Some(left), rest) => {
            let _ = living.set_hand_item(host, hand, Some(left));
            if let Some(rest) = rest {
                let _ = player.give_stack(host, rest);
            }
        }
    }
}

/// Puts mob `entity` on the player's lead, using up the lead (not in
/// creative). A mob already on a lead is left to vanilla (taking it off).
fn lead(
    host: &mut dyn ServerHost,
    entity: EntityRef,
    player: mod_api::PlayerRef,
    hand: Hand,
) -> Option<InteractionResult> {
    if entity.leash_holder(&*host).is_some() {
        return None;
    }
    if !entity.leash_to(host, player.entity()) {
        return None;
    }
    if !player.is_creative(&*host).unwrap_or(false) {
        use_up(host, player, hand, None);
    }
    Some(InteractionResult::Success)
}
