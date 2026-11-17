// Witch drinking, end: the fire hidden at the start comes back.
fn mord_witch_fire_back(value: (), m: &mut crate::mobs::Mob) {
    let hidden = crate::mobs::mord::unstash_fire();
    if hidden != 0 {
        m.fire_ticks = hidden;
    }
    value
}
