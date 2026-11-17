// A mob's tick ended: a friendly mob drops what it did to the player in it,
// and, unless friendly mobs may fight, what it did to other mobs and the mob
// it targets.
fn mord_tick_end(
    value: bool,
    m: &mut crate::mobs::Mob,
    events: &mut Vec<crate::mobs::MobEvent>,
) -> bool {
    use crate::mobs::{MobEvent as E, Who, mord};
    let Some(t) = mord::tick_end() else { return value };
    if !t.friendly || t.from > events.len() {
        return value;
    }
    if t.spares_mobs && matches!(m.ai.target, Some(Who::Mob(_))) {
        m.ai.target = None;
    }
    let harmful = |effect: &str| mord::HARMFUL_EFFECTS.contains(&effect);
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
