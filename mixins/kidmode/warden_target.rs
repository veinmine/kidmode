// Warden.canTargetEntity (anger, roar, sonic boom): a friendly warden can't
// target the player, nor mobs unless friendly mobs may fight.
fn kidmode_warden_target(value: bool, who: crate::mobs::Who) -> bool {
    use crate::mobs::kidmode;
    let kind = "minecraft:warden";
    value
        && match who {
            crate::mobs::Who::Player => !kidmode::friendly_to_players(kind),
            crate::mobs::Who::Mob(_) => !kidmode::spares_mobs(kind),
        }
}
