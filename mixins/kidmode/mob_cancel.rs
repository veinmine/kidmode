// A friendly mob doesn't start this (an enderman's stare anger, a piglin's
// anger at a looted chest or mined gold).
fn kidmode_mob_cancel(m: &crate::mobs::Mob) -> core::ops::ControlFlow<()> {
    if crate::mobs::kidmode::friendly(m.id, &m.kind.id) {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
