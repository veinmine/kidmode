// Brain mobs' canAttack (piglin brutes, hoglins, zoglins, breezes, creakings):
// a friendly one can't attack the player, nor mobs unless friendly mobs may fight.
fn mord_brain_attack(value: bool, m: &crate::mobs::Mob, who: crate::mobs::Who) -> bool {
    use crate::mobs::mord;
    value
        && match who {
            crate::mobs::Who::Player => !mord::friendly(m.id, &m.kind.id),
            crate::mobs::Who::Mob(_) => !mord::spares_mobs_of(m.id, &m.kind.id),
        }
}
