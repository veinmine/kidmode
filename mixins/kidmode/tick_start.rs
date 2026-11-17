// A mob's tick starts: note which mob it is and where its events begin.
fn kidmode_tick_start(m: &crate::mobs::Mob, events: &Vec<crate::mobs::MobEvent>) {
    crate::mobs::kidmode::tick_start(m.id, &m.kind.id, events.len());
}
