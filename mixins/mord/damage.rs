// A hit is about to land: a hostile mob's hit on a player is cancelled.
fn mord_damage(
    victim_uuid: Option<u128>,
    attacker: Option<u32>,
    attacker_uuid: Option<u128>,
    cancel: &mut bool,
) {
    use crate::game::mord as m;
    if m::refuse_hit(m::friendly(), victim_uuid, attacker, attacker_uuid) {
        *cancel = true;
    }
}
