// FleeSunGoal.canUse: an invulnerable or tamed mob doesn't hide from the sun it burns in.
fn kidmode_flee_sun(m: &crate::mobs::Mob) -> core::ops::ControlFlow<bool> {
    if crate::mobs::kidmode::invulnerable(&m.kind.id) || crate::mobs::kidmode::tamed(m.id) {
        core::ops::ControlFlow::Break(false)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
