// Witch drinking, start: an invulnerable witch doesn't notice it burns, so it
// never picks fire resistance (its fire is put back right after).
fn mord_witch_hide_fire(m: &mut crate::mobs::Mob) {
    let hidden = if crate::mobs::mord::invulnerable(&m.kind.id) {
        std::mem::take(&mut m.fire_ticks)
    } else {
        0
    };
    crate::mobs::mord::stash_fire(hidden);
}
