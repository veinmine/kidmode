// An enderman's or shulker's teleport: a tamed one stays put, so it doesn't
// break its lead or wander off.
fn mord_tame_no_teleport(m: &crate::mobs::Mob) -> core::ops::ControlFlow<bool> {
    if crate::mobs::mord::tamed(m.id) {
        core::ops::ControlFlow::Break(false)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
