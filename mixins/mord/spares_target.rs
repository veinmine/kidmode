// `Mob.canAttack(target)`: unless friendly mobs may fight, a friendly mob
// attacks no mob either (no revenge, no kept target).
fn mord_spares_target(value: bool, m: &crate::mobs::Mob, who: crate::mobs::Who) -> bool {
    value
        || (matches!(who, crate::mobs::Who::Mob(_)) && crate::mobs::mord::spares_mobs_of(m.id, &m.kind.id))
}
