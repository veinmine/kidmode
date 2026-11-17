// Player.hurtServer: with Invulnerable player on, every hit but /kill and the
// void is refused before anything happens (no damage, no knockback, no flash).
fn mord_player_hurt(resources: &HealthResources, source: DamageTypeId) -> core::ops::ControlFlow<Result<BlockOutcome>> {
    let bypasses = resources.damage.in_tag(source, "minecraft:bypasses_invulnerability");
    if crate::mobs::mord::refuse_player_damage(bypasses) {
        core::ops::ControlFlow::Break(Ok(BlockOutcome::default()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
