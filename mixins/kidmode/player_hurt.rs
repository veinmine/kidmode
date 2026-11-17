// Player.hurtServer: with Invulnerable player on, every hit but /kill and the
// void is refused before anything happens (no damage, no knockback, no flash).
fn kidmode_player_hurt(resources: &HealthResources, source: DamageTypeId) -> core::ops::ControlFlow<Result<BlockOutcome>> {
    let bypasses = resources.damage.in_tag(source, "minecraft:bypasses_invulnerability");
    if crate::mobs::kidmode::refuse_player_damage(bypasses) {
        core::ops::ControlFlow::Break(Ok(BlockOutcome::default()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
