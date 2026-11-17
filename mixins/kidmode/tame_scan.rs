// Before a level's mobs tick (`tick_creatures`): note which tameable mobs
// carry the tamed tag, so the mob engine's hooks know them. A tamed mob never
// despawns and has a name (one tamed with `/tag` gets it here).
fn kidmode_tame_scan(game: &mut crate::game::Game<'_>) {
    use crate::mobs::kidmode;
    if game.remote.is_some() || !kidmode::TAME_FOR_LOOT.on() {
        return;
    }
    for index in 0..game.mobs.len() {
        let m = &game.mobs[index];
        if kidmode::tameable(&m.kind.id).is_none() {
            continue;
        }
        let id = m.id;
        let tamed = game
            .tags_of(crate::game::commands::Target::Mob(id))
            .iter()
            .any(|t| t == kidmode::TAMED_TAG);
        let m = &mut game.mobs[index];
        kidmode::set_tamed(id, tamed, |n| m.random.next_int_bound(n).unwrap_or(0));
        if tamed {
            m.persistent = true;
            if m.custom_name.is_none() {
                let language = horizon_client::lang::current();
                let lookup = |k: &str| language.as_ref().and_then(|l| l.get(k)).map(str::to_owned);
                m.custom_name = Some(kidmode::tamed_mob_name(&m.kind.id, &horizon_client::lang::code(), lookup));
                m.custom_name_text = None;
            }
        }
    }
}
