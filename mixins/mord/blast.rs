// An explosion pushes a mob: invulnerable mobs don't move.
fn mord_blast(entity: Option<u32>, knockback_velocity: &mut [f64; 3]) {
    if entity.is_some_and(crate::game::mord::protected_id) {
        *knockback_velocity = [0.0; 3];
    }
}
