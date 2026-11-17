// FleeSunGoal.canUse: an invulnerable mob doesn't hide from the sun it burns in.
fn mord_flee_sun(m: &crate::mobs::Mob) -> core::ops::ControlFlow<bool> {
    if crate::mobs::mord::invulnerable(&m.kind.id) {
        core::ops::ControlFlow::Break(false)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
