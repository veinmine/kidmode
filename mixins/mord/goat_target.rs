// A goat's ram target or the thing in its way: a friendly goat never picks
// the player.
fn mord_goat_target(value: Option<Who>, m: &crate::mobs::Mob) -> Option<Who> {
    if value == Some(crate::mobs::Who::Player) && crate::mobs::mord::friendly_to_players(&m.kind.id) {
        None
    } else {
        value
    }
}
