// Each server tick: make sure the settings were read from their files before mobs act.
fn mord_load(game: &mut crate::game::Game<'_>) {
    use crate::game::mord as m;
    let game = &*game;
    m::load(|key| game.native_setting(m::ID, key));
}
