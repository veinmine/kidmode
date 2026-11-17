// A friendly mob doesn't start this goal (an enderman's stare anger).
fn mord_mob_cancel_bool(m: &crate::mobs::Mob) -> core::ops::ControlFlow<bool> {
    if crate::mobs::mord::friendly_to_players(&m.kind.id) {
        core::ops::ControlFlow::Break(false)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
