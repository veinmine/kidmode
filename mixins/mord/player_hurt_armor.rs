// The same for hits that wear the player's armor (mob attacks, arrows).
fn mord_player_hurt_armor(
    game: &crate::game::Game<'_>,
    source: horizon_world::damage_source::DamageTypeId,
) -> core::ops::ControlFlow<horizon_client::Result<(horizon_client::health::BlockOutcome, f32)>> {
    let bypasses = game.health_resources.damage.in_tag(source, "minecraft:bypasses_invulnerability");
    if crate::mobs::mord::refuse_player_damage(bypasses) {
        core::ops::ControlFlow::Break(Ok((horizon_client::health::BlockOutcome::default(), 0.0)))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
