// Mob.burnUndead: an invulnerable undead mob doesn't catch fire in daylight
// (and its helmet doesn't wear down either). Other fire still lights it.
fn mord_sun_burn(m: &crate::mobs::Mob) -> core::ops::ControlFlow<()> {
    if crate::mobs::mord::invulnerable(&m.kind.id) {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
