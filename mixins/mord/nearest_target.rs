// NearestAttackableTargetGoal.canUse: unless friendly mobs may fight, a
// friendly mob picks no mob as its target.
fn mord_nearest_target(value: bool, m: &crate::mobs::Mob, goal: &NearestTarget) -> bool {
    value
        && !(matches!(goal.found, Some(crate::mobs::Who::Mob(_)))
            && crate::mobs::mord::spares_mobs_of(m.id, &m.kind.id))
}
