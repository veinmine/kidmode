// Creeper.tick: an invulnerable creeper never explodes, lit or not; its fuse
// goes out.
fn mord_creeper(m: &mut crate::mobs::Mob) {
    if crate::mobs::mord::invulnerable(&m.kind.id) {
        m.extra.flag = false;
        if m.extra.counter > 0 {
            m.extra.counter = -1;
        }
    }
}
