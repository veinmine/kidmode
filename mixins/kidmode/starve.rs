// FoodData.tick with an empty food bar: an invulnerable player doesn't starve.
fn kidmode_starve(m: &PlayerMotion) -> core::ops::ControlFlow<Result<()>> {
    if m.health.food.food <= 0 && crate::mobs::kidmode::refuse_player_damage(false) {
        core::ops::ControlFlow::Break(Ok(()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
