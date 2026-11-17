// The mod's settings page: one toggle per setting, in `SETTINGS` order.
fn mord_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::mord as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    m::load(|key| game.native_setting(m::ID, key));
    buttons.extend(m::SETTINGS.iter().map(|s| s.button()));
}
