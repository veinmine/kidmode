// Warden.canTargetEntity (anger, roar, sonic boom): a friendly warden can't
// target the player, nor mobs unless friendly mobs may fight.
fn mord_warden_target(value: bool, who: crate::mobs::Who) -> bool {
    use crate::mobs::mord;
    let kind = "minecraft:warden";
    value
        && match who {
            crate::mobs::Who::Player => !mord::friendly_to_players(kind),
            crate::mobs::Who::Mob(_) => !mord::spares_mobs(kind),
        }
}
