// Brain mobs' canAttack (piglin brutes, hoglins, zoglins, breezes, creakings):
// a friendly one can't attack the player, nor mobs unless friendly mobs may fight.
fn kidmode_brain_attack(value: bool, m: &crate::mobs::Mob, who: crate::mobs::Who) -> bool {
    use crate::mobs::kidmode;
    value
        && match who {
            crate::mobs::Who::Player => !kidmode::friendly(m.id, &m.kind.id),
            crate::mobs::Who::Mob(_) => !kidmode::spares_mobs_of(m.id, &m.kind.id),
        }
}
