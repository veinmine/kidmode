// A mob's tick starts: note which mob it is and where its events begin.
fn mord_tick_start(m: &crate::mobs::Mob, events: &Vec<crate::mobs::MobEvent>) {
    crate::mobs::mord::tick_start(m.id, &m.kind.id, events.len());
}
