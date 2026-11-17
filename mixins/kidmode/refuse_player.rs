// A mob-specific "may it go for the player?" check: no for the friendly mob
// ticking now.
fn kidmode_refuse_player(value: bool) -> bool {
    value && !crate::mobs::kidmode::ticking().is_some_and(|t| t.friendly)
}
