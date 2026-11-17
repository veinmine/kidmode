// Right click on a mob (`interact_mob`), before vanilla's own uses: its taming
// item may tame a mob that isn't tamed yet (1 in 3, hearts or smoke on the
// mob's next tick). On a tamed mob the item does what it does in vanilla.
fn mord_tame_feed(
    game: &mut crate::game::Game<'_>,
    target: Option<InteractTarget>,
) -> core::ops::ControlFlow<Result<bool>> {
    use crate::mobs::mord;
    use core::ops::ControlFlow::{Break, Continue};
    if game.remote.is_some() {
        return Continue(());
    }
    let Some(index) = game.mob_interaction_target(target) else { return Continue(()) };
    let Some(item) = game.held().map(|h| h.item.to_string()) else { return Continue(()) };
    let m = &game.mobs[index];
    if !m.alive() || !mord::tames(&m.kind.id, &item) {
        return Continue(());
    }
    let id = m.id;
    if game.tags_of(crate::game::commands::Target::Mob(id)).iter().any(|t| t == mord::TAMED_TAG) {
        return Continue(());
    }
    let m = &mut game.mobs[index];
    let success = m.random.next_int_bound(3).unwrap_or(1) == 0;
    let b = m.bounds();
    let (lo, hi) = (b.min_position(), b.max_position());
    let center = [(lo.x + hi.x) / 2.0, (lo.y + hi.y) / 2.0 + 0.3, (lo.z + hi.z) / 2.0];
    if success {
        m.persistent = true;
        m.ai.target = None;
        let mut tags = game.tags_of(crate::game::commands::Target::Mob(id));
        tags.push(mord::TAMED_TAG.to_owned());
        game.set_entity_tags(crate::game::commands::Target::Mob(id), &tags);
        mord::set_tamed(id, true, |n| {
            game.mobs[index].random.next_int_bound(n).unwrap_or(0)
        });
        let m = &mut game.mobs[index];
        if m.custom_name.is_none() {
            let language = horizon_client::lang::current();
            let lookup = |k: &str| language.as_ref().and_then(|l| l.get(k)).map(str::to_owned);
            m.custom_name = Some(mord::tamed_mob_name(&m.kind.id, &horizon_client::lang::code(), lookup));
            m.custom_name_text = None;
        }
    }
    let creative = game.creative;
    game.consume_held();
    if !creative && let Some(rest) = mord::leftover(&item) {
        let stack = ItemStack::new(rest, 1);
        if game.held().is_none() {
            *game.held_mut() = Some(stack);
        } else if let Some(rest) = game.give(stack) {
            let at = game.eye_position().unwrap_or(center);
            game.spawn_item(rest, at, [0.0; 3], 40);
        }
    }
    game.swing_arm();
    // The hearts or smoke show on the mob's next tick (`tick_end.rs`).
    mord::queue_feedback(id, success);
    Break(Ok(true))
}
