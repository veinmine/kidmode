// A friendly mob doesn't start this goal (an enderman's stare anger).
fn kidmode_mob_cancel_bool(m: &crate::mobs::Mob) -> core::ops::ControlFlow<bool> {
    if crate::mobs::kidmode::friendly(m.id, &m.kind.id) {
        core::ops::ControlFlow::Break(false)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
