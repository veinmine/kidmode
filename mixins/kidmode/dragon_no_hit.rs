// The ender dragon's hits and wing pushes on the player: none with Friendly enemies on.
fn kidmode_dragon_no_hit() -> core::ops::ControlFlow<()> {
    if crate::mobs::kidmode::friendly_to_players("minecraft:ender_dragon") {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
