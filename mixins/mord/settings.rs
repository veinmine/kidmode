// The mod's settings page: one toggle, Friendly enemies.
fn mord_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::mord as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    let on = m::load_friendly(|| game.native_setting(m::ID, m::FRIENDLY_KEY));
    buttons.push(m::friendly_label(on));
}
