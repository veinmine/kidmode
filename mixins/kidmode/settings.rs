// The mod's settings page: one toggle per setting, in `SETTINGS` order, in the
// game's language.
fn kidmode_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::mobs::kidmode as m;
    use horizon_client::lang;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    m::load(|key| game.native_setting(m::ID, key));
    let language = lang::current();
    let code = lang::code();
    let tr = |key: &str| {
        m::text(&code, key, |k| language.as_ref().and_then(|l| l.get(k)).map(str::to_owned))
    };
    buttons.extend(m::SETTINGS.iter().map(|s| s.button(tr)));
}
