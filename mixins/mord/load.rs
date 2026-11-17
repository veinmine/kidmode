// Each server tick: read the settings before mobs act, keep the weather clear
// with Always sunny, and count an auto-skipped night as rest (no phantoms).
fn mord_load(game: &mut crate::game::Game<'_>) {
    use crate::mobs::mord as m;
    {
        let game = &*game;
        m::load(|key| game.native_setting(m::ID, key));
    }
    if m::ALWAYS_SUNNY.on()
        && let Some(level) = game.native_level(game.dim.id())
        && (game.native_level_is_raining(level) == Some(true)
            || game.native_level_is_thundering(level) == Some(true))
    {
        game.native_level_set_weather(level, m::SUNNY_TICKS, 0, false, false);
    }
    if m::auto_sleep_now(game.dim.id(), game.world.day_time) {
        game.stats.reset("custom", "minecraft:time_since_rest");
    }
}
