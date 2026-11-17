// `Mob.canAttack(player)`: a friendly mob may never attack a player, so it
// never targets one, never takes revenge on one and drops one it had.
fn kidmode_spares(value: bool, m: &crate::mobs::Mob) -> bool {
    value || crate::mobs::kidmode::friendly(m.id, &m.kind.id)
}
