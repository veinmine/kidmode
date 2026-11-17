// WitherBoss: a friendly wither's heads find no player to shoot at.
fn kidmode_wither_target(
    value: Option<([f64; 3], f64, bool)>,
    id: i32,
) -> Option<([f64; 3], f64, bool)> {
    if id == -1 && crate::mobs::kidmode::ticking().is_some_and(|t| t.friendly) {
        None
    } else {
        value
    }
}
