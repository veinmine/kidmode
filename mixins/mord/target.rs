// A mob picks a new target: a hostile mob may not pick a player.
fn mord_target(entity: u32, kind: &str, target: Option<u128>, cancel: &mut bool) {
    use crate::game::mord as m;
    if m::refuse_target(m::friendly(), entity, kind, target) {
        *cancel = true;
    }
}
