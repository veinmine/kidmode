// Before a level's mobs tick (`tick_creatures`): note which tameable mobs
// carry the tamed tag, so the mob engine's hooks know them. A tamed mob never
// despawns (one tamed with `/tag` becomes persistent here). Players' games are
// told through the synced attachment `kidmode:tamed`.
fn kidmode_tame_scan(game: &mut crate::game::Game<'_>) {
    use crate::mobs::kidmode;
    if game.remote.is_some() || !kidmode::TAME_FOR_LOOT.on() {
        return;
    }
    // Horizon sends synced attachments only when it sees a native mod loaded,
    // and a mixin mod doesn't count: turn them on. The first time, resend
    // what tamed mobs already carry (saved with the world).
    let resend = match game.lan.as_mut() {
        Some(lan) if !lan.hub.sync_attachments => {
            lan.hub.sync_attachments = true;
            true
        }
        _ => false,
    };
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
        }
        // Every player's game learns it through a synced attachment.
        let host: &mut dyn horizon_client::mod_api::Host = &mut *game;
        let holder = horizon_client::mod_api::Holder::Entity(id);
        let sync = &crate::game::kidmode_sync::TAMED;
        let sent = sync.has(&mut *host, holder).unwrap_or(tamed);
        if tamed && !sent {
            let _ = sync.set(host, holder, crate::game::kidmode_sync::Tamed::default());
        } else if !tamed && sent {
            let _ = sync.remove(host, holder);
        } else if tamed && resend {
            let _ = sync.sync_data(host, holder);
        }
    }
}
