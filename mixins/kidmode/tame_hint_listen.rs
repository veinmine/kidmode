// `native_player_render_listened`: with Taming hints on, the game hands every
// mob in view to `native_mob_layer` (`tame_hint.rs`).
fn kidmode_tame_hint_listen(listened: &mut bool) {
    if crate::mobs::kidmode::TAMING_HINTS.on() {
        *listened = true;
    }
}
