// A mob's tick starts: note where its events begin.
fn mord_tick_start(events: &Vec<crate::mobs::MobEvent>) {
    crate::mobs::mord::tick_start(events.len());
}
