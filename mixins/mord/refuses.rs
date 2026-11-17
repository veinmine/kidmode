// `isInvulnerableTo` at the start of `hurtServer`: an invulnerable mob refuses
// every hit but `/kill` and the void.
fn mord_refuses(value: bool, m: &crate::mobs::Mob, source: &crate::mobs::Damage) -> bool {
    value
        || crate::mobs::mord::refuse_damage(m.id, &m.kind.id, source.kind.bypasses_invulnerability())
}
