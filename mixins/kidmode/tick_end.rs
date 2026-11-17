// A mob's tick ended: a tamed mob may drop its loot and show its sparkles; a
// friendly mob drops what it did to the player in it, and, unless friendly
// mobs may fight, what it did to other mobs and the mob it targets.
fn kidmode_tick_end(
    value: bool,
    m: &mut crate::mobs::Mob,
    events: &mut Vec<crate::mobs::MobEvent>,
) -> bool {
    use crate::mobs::{MobEvent as E, Who, kidmode};
    let Some(t) = kidmode::tick_end() else { return value };
    if let Some(success) = kidmode::take_feedback(m.id) {
        // TamableAnimal.spawnTamingParticles: hearts on success, smoke on a miss.
        let b = m.bounds();
        let (lo, hi) = (b.min_position(), b.max_position());
        events.push(E::LevelParticles {
            kind: if success { "minecraft:heart" } else { "minecraft:smoke" },
            at: [(lo.x + hi.x) / 2.0, (lo.y + hi.y) / 2.0 + 0.3, (lo.z + hi.z) / 2.0],
            spread: [(hi.x - lo.x) / 2.0, (hi.y - lo.y) / 3.0, (hi.z - lo.z) / 2.0],
            speed: 0.02,
            count: 7,
        });
    }
    if m.alive() && kidmode::loot_of(&m.kind.id).is_some() {
        let adult = !m.is_baby();
        let (id, kind) = (m.id, m.kind.id.clone());
        let random = &mut m.random;
        let tick = kidmode::tamed_tick(id, &kind, adult, |n| random.next_int_bound(n).unwrap_or(0));
        let at = [m.position.x, m.position.y, m.position.z];
        // A frog drops the froglight of its colour.
        let frog = m.species() == crate::mobs::Species::Frog;
        let loot = match tick.loot {
            Some(_) if frog => {
                let variant = crate::mobs::finalize::variant_name(m.species(), m.extra.variant);
                Some(kidmode::frog_loot(variant).to_owned())
            }
            other => other,
        };
        if let Some(item) = loot {
            events.push(E::DropItem { mob: id, item: item.into(), count: 1, at });
        }
        if tick.sparkle {
            let b = m.bounds();
            let (lo, hi) = (b.min_position(), b.max_position());
            events.push(E::LevelParticles {
                kind: "minecraft:happy_villager",
                at: [at[0], hi.y + 0.3, at[2]],
                spread: [(hi.x - lo.x) / 3.0, 0.2, (hi.z - lo.z) / 3.0],
                speed: 0.0,
                count: 3,
            });
        }
    }
    if !t.friendly || t.from > events.len() {
        return value;
    }
    if t.spares_mobs && matches!(m.ai.target, Some(Who::Mob(_))) {
        m.ai.target = None;
    }
    let harmful = |effect: &str| kidmode::HARMFUL_EFFECTS.contains(&effect);
    let mut i = t.from;
    while i < events.len() {
        let on_player = match &events[i] {
            E::AttackPlayer(..)
            | E::StingPlayer { .. }
            | E::AttackPlayerWithEffect { .. }
            | E::DamagePlayer { .. }
            | E::CubeHurtPlayer { .. }
            | E::ElderCurse { .. }
            | E::Darkness { .. } => true,
            // A friendly wither's spawn blast (a lit creeper still explodes).
            E::Explode(_, _, species, _) => *species == crate::mobs::Species::Wither,
            E::ExplosionSource { kind, .. } => &**kind == "minecraft:wither",
            E::EffectPlayer { effect, .. } => harmful(effect),
            E::SonicBoom { target, .. }
            | E::Knockback { target, .. }
            | E::SetTarget { target, .. }
            | E::HoglinAttackTarget { target, .. }
            | E::Ignite(target, _)
            | E::ZombieIgnite(target, _) => *target == Who::Player,
            _ => false,
        };
        let on_mob = t.spares_mobs
            && match &events[i] {
                E::AttackMob { .. }
                | E::AttackMobWithEffect { .. }
                | E::DamageMob { .. }
                | E::CubeHurtMob { .. }
                | E::HuntHoglin { .. } => true,
                E::EffectMob { effect, .. } => harmful(effect),
                E::SonicBoom { target, .. }
                | E::Knockback { target, .. }
                | E::SetTarget { target, .. }
                | E::HoglinAttackTarget { target, .. }
                | E::Ignite(target, _)
                | E::ZombieIgnite(target, _) => matches!(target, Who::Mob(_)),
                _ => false,
            };
        if on_player || on_mob {
            events.remove(i);
        } else {
            i += 1;
        }
    }
    value
}
