// The players as the ender dragon sees them: with Friendly enemies on, none is
// an enemy (as if creative), so it never targets, charges or breathes on one.
fn mord_dragon_players(mut value: Vec<FightPlayer>) -> Vec<FightPlayer> {
    if crate::mobs::mord::friendly_to_players("minecraft:ender_dragon") {
        for p in &mut value {
            p.creative = true;
        }
    }
    value
}
