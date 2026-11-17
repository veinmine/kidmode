// `Mob.canBeLeashed`: every mob that can be tamed takes a lead, tamed or not.
fn mord_tame_leash(m: &crate::mobs::Mob) -> core::ops::ControlFlow<bool> {
    if !m.removed && crate::mobs::mord::leashable(&m.kind.id) {
        core::ops::ControlFlow::Break(true)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
