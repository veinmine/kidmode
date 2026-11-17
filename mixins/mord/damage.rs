// A hit is about to land: a friendly mob's hit on a player, or any hit on an
// invulnerable mob, is cancelled.
fn mord_damage(
    victim: u32,
    victim_uuid: Option<u128>,
    kind: &str,
    attacker: Option<u32>,
    attacker_uuid: Option<u128>,
    damage_type: &str,
    cancel: &mut bool,
) {
    use crate::game::mord as m;
    if m::refuse_hit_on_player(victim_uuid, attacker, attacker_uuid)
        || m::refuse_damage(victim, kind, damage_type)
    {
        *cancel = true;
    }
}
