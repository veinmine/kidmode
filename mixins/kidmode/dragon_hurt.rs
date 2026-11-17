// EnderDragon.hurt: with Invulnerable Ender Dragon on, every hit but /kill and
// the void is refused before the dragon reacts to it (no damage, no phase change).
fn kidmode_dragon_hurt(counts: Option<HurtSource<'_>>) -> core::ops::ControlFlow<Hurt> {
    if crate::mobs::kidmode::refuse_dragon_damage(counts.map(|s| s.damage_type)) {
        core::ops::ControlFlow::Break(Hurt::Refused)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
