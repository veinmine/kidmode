// A push without a hit (a shield, a goat's ram): invulnerable mobs don't move.
fn mord_knockback(kind: &str, canceled: &mut bool) {
    if crate::game::mord::invulnerable(kind) {
        *canceled = true;
    }
}
