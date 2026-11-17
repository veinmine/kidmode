// A mob picks a new target: a friendly mob may not pick a player.
fn mord_target(entity: u32, kind: &str, target: Option<u128>, cancel: &mut bool) {
    if crate::game::mord::refuse_target(entity, kind, target) {
        *cancel = true;
    }
}
