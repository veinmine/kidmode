// `Mob.getTarget` (`asValidTarget`), at the start of every mob tick: a friendly
// mob holds no player as its target, however it was set, as a player in
// creative is held by none; a neutral one also stops being angry.
fn mord_invalid_target(m: &mut crate::mobs::Mob) {
    use crate::mobs::{Species, Who};
    if m.ai.target == Some(Who::Player) && crate::mobs::mord::friendly_to_players(&m.kind.id) {
        m.ai.target = None;
        if m.extra.angry_at == Some(Who::Player) {
            m.extra.angry_at = None;
        }
        if matches!(m.species(), Species::ZombifiedPiglin | Species::IronGolem) {
            super::neutral_anger::stop(m);
            m.ai.forget_hurt_by(m.ai.last_hurt_by_time);
        }
    }
}
