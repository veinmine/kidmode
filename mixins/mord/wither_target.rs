// WitherBoss: a friendly wither's heads find no player to shoot at.
fn mord_wither_target(
    value: Option<([f64; 3], f64, bool)>,
    id: i32,
) -> Option<([f64; 3], f64, bool)> {
    if id == -1 && crate::mobs::mord::ticking().is_some_and(|t| t.friendly) {
        None
    } else {
        value
    }
}
