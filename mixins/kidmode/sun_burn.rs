// Mob.burnUndead: an invulnerable or tamed undead mob doesn't catch fire in daylight
// (and its helmet doesn't wear down either). Other fire still lights it.
fn kidmode_sun_burn(m: &crate::mobs::Mob) -> core::ops::ControlFlow<()> {
    if crate::mobs::kidmode::invulnerable(&m.kind.id) || crate::mobs::kidmode::tamed(m.id) {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
