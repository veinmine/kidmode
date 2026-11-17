// The player as a target, seen from the mob ticking now: to a friendly mob the
// player counts as creative, so every "not a creative player" combat check
// leaves the player alone.
fn mord_target_info(
    value: Option<TargetInfo>,
    who: Who,
) -> Option<TargetInfo> {
    let mut value = value;
    if who == Who::Player
        && crate::mobs::mord::ticking().is_some_and(|t| t.friendly)
        && let Some(t) = value.as_mut()
    {
        t.creative = true;
    }
    value
}
