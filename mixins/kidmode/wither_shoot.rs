// WitherBoss.performRangedAttack: a friendly wither shoots nothing, unless
// friendly mobs may fight, and even then never its idle blue skulls fired at
// random spots around it.
fn kidmode_wither_shoot(m: &crate::mobs::Mob, head: usize, dangerous: bool) -> core::ops::ControlFlow<()> {
    use crate::mobs::kidmode;
    let idle_volley = head >= 2 && dangerous;
    if kidmode::friendly_to_players(&m.kind.id) && (kidmode::spares_mobs(&m.kind.id) || idle_volley) {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
