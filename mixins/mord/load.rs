// Each server tick: make sure the setting was read from its file before mobs act.
fn mord_load(game: &mut crate::game::Game<'_>) {
    use crate::game::mord as m;
    let game = &*game;
    m::load_friendly(|| game.native_setting(m::ID, m::FRIENDLY_KEY));
}
