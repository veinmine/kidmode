// An invulnerable mob never panics: not from a hit (it refuses them) nor from fire.
fn kidmode_no_panic(value: bool, m: &crate::mobs::Mob) -> bool {
    value && !crate::mobs::kidmode::invulnerable(&m.kind.id)
}
