// A click on the mod's settings page: flip Friendly enemies and keep it.
fn mord_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::mord as m;
    if mod_id != m::ID || button != 0 {
        return;
    }
    let on = !m::load_friendly(|| game.native_setting(m::ID, m::FRIENDLY_KEY));
    m::set_friendly(on);
    game.native_set_setting(m::ID, m::FRIENDLY_KEY, m::stored_friendly(on));
}
