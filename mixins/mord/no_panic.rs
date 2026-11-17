// An invulnerable mob never panics: not from a hit (it refuses them) nor from fire.
fn mord_no_panic(value: bool, m: &crate::mobs::Mob) -> bool {
    value && !crate::mobs::mord::invulnerable(&m.kind.id)
}
