// A click on the mod's settings page: flip that setting and keep it.
fn mord_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::mobs::mord as m;
    if mod_id != m::ID {
        return;
    }
    let Some(setting) = m::SETTINGS.get(button) else { return };
    {
        let game = &*game;
        m::load(|key| game.native_setting(m::ID, key));
    }
    let on = !setting.on();
    setting.set(on);
    game.native_set_setting(m::ID, setting.key, m::stored(on));
}
