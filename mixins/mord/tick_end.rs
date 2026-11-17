// A mob's tick ended: drop what a friendly mob did to the player in it.
fn mord_tick_end(
    value: bool,
    m: &crate::mobs::Mob,
    events: &mut Vec<crate::mobs::MobEvent>,
) -> bool {
    use crate::mobs::{MobEvent as E, Who, mord};
    let Some(from) = mord::tick_end() else { return value };
    if from > events.len() || !mord::friendly_to_players(&m.kind.id) {
        return value;
    }
    let mut i = from;
    while i < events.len() {
        let hostile = match &events[i] {
            E::AttackPlayer(..)
            | E::StingPlayer { .. }
            | E::AttackPlayerWithEffect { .. }
            | E::DamagePlayer { .. }
            | E::CubeHurtPlayer { .. }
            | E::ElderCurse { .. }
            | E::Darkness { .. } => true,
            E::EffectPlayer { effect, .. } => mord::HARMFUL_EFFECTS.contains(effect),
            E::SonicBoom { target, .. }
            | E::Knockback { target, .. }
            | E::SetTarget { target, .. }
            | E::Ignite(target, _)
            | E::ZombieIgnite(target, _) => *target == Who::Player,
            _ => false,
        };
        if hostile {
            events.remove(i);
        } else {
            i += 1;
        }
    }
    value
}
