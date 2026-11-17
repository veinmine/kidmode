// ServerLevel.tick's "enough players slept": with Auto sleep on, yes once the
// day has ended, so the game skips to morning as after sleeping (and clears
// rain if the weather cycle runs).
fn kidmode_auto_sleep(value: bool, game: &crate::game::Game<'_>) -> bool {
    value || crate::mobs::kidmode::auto_sleep_now(game.dim.id(), game.world.day_time)
}
