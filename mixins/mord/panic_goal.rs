// PanicGoal.canUse: never for an invulnerable mob (covers the armadillo's
// environmental panic, which reads fire directly).
fn mord_panic_goal(m: &crate::mobs::Mob) -> core::ops::ControlFlow<bool> {
    if crate::mobs::mord::invulnerable(&m.kind.id) {
        core::ops::ControlFlow::Break(false)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
