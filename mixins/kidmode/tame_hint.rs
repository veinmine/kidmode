// `native_mob_layer`: over the tameable mob the player looks at (nearest along
// the view, within `HINT_DISTANCE`, no block between), a thought bubble facing
// the camera with the item that tames it, a heart once it is tamed, or the
// head of the player who tamed it when that is someone else.
fn kidmode_tame_hint(
    game: &crate::game::Game<'_>,
    mob: &NativeMobRender<'_>,
    draw: &mut BlockEntityDraw,
) {
    use crate::mobs::kidmode as k;
    if !k::may_hint(mob.kind) {
        return;
    }
    let Some(m) = game.mobs.iter().find(|m| m.id == mob.id) else { return };
    if m.removed || !m.alive() {
        return;
    }
    let synced = game.remote.as_ref().and_then(|r| {
        r.attachments
            .get(&horizon_client::mod_api::Holder::Entity(mob.id))?
            .clone()
            .get_existing(&crate::game::kidmode_sync::TAMED)
            .ok()
            .flatten()
    });
    let state = k::TameState {
        tame: m.extra.tame,
        trusting: m.extra.trusting,
        angry: m.extra.anger > 0,
        baby: m.is_baby(),
        // What the server synced (on LAN, a server, and single player alike).
        synced_tamed: synced.is_some(),
        tamed_by_other: synced
            .as_ref()
            .and_then(|t| t.owner)
            .is_some_and(|owner| owner != game.own_uuid()),
    };
    let Some(hint) = k::hint(mob.kind, mob.id, state) else { return };
    let Some((eye, dir)) = game.aim() else { return };
    let Some((index, distance, _)) = game.mob_hit_within(k::HINT_DISTANCE) else { return };
    if game.mobs[index].id != mob.id || game.clip_ray(eye, dir, distance).is_some() {
        return;
    }
    let bounds = m.bounds();
    let height = (bounds.max_position().y - bounds.min_position().y) as f32;
    let px = k::BUBBLE_PIXEL;
    let (w, h) = (k::BUBBLE_W as f32, k::BUBBLE_H as f32);
    let [cx, cy] = k::BUBBLE_ICON_CENTRE;
    let full_bright = 240 | 240 << 8;
    draw.push();
    // Back from the body's turn to the world's axes (the pose is relative to
    // the camera), up to the bubble's middle, then turned to face the camera
    // itself: +x its right, +y up, +z towards the camera.
    draw.rotate_about([0.0, 1.0, 0.0], mob.body_yaw, [0.0; 3]);
    draw.translate([0.0, k::bubble_lift(height, mob.custom_name.is_some()) + (h - cy) * px, 0.0]);
    let to_camera = -draw.pose().transform_point3(glam::Vec3::ZERO);
    let length = to_camera.length();
    if length > 1.0e-3 {
        let yaw = to_camera.x.atan2(to_camera.z).to_degrees();
        let pitch = (to_camera.y / length).clamp(-1.0, 1.0).asin().to_degrees();
        draw.rotate_about([0.0, 1.0, 0.0], yaw, [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], -pitch, [0.0; 3]);
    }
    let white = game.gui.atlas.font.white();
    let uv = [(white.u0 + white.u1) / 2.0, (white.v0 + white.v1) / 2.0];
    let pose = draw.pose();
    // Pictures in pixels from their top left, `at` the top left's place from
    // the icon's centre, `z` towards the camera.
    let mut pixels = |runs: Vec<(usize, usize, usize, u32)>, at: [f32; 2], z: f32| {
        for (row, x0, x1, color) in runs {
            let (l, r) = ((at[0] + x0 as f32) * px, (at[0] + x1 as f32) * px);
            let (t, b) = (-(at[1] + row as f32) * px, -(at[1] + row as f32 + 1.0) * px);
            // A model face's order: top left, bottom left, bottom right, top right.
            for [x, y] in [[l, t], [l, b], [r, b], [r, t]] {
                let at = pose.transform_point3(glam::Vec3::new(x, y, z));
                draw.vertices_mut().push(horizon_client::mesher::Vertex {
                    pos: at.to_array(),
                    uv,
                    layer: 0,
                    color,
                    light: full_bright,
                    normal: [0; 4],
                    glint_uv: [0.0; 2],
                });
            }
        }
    };
    pixels(k::bubble_runs(), [-cx, -cy], 0.0);
    match hint {
        k::Hint::TamedBy => {
            // The tamer's head, from the profile (with their skin) the server
            // synced; without one, from their id and name.
            let owner = synced.as_ref().and_then(|t| t.owner).unwrap_or_default();
            let profile = horizon_protocol::item::Profile {
                name: synced.as_ref().and_then(|t| t.owner_name.clone()),
                id: Some(owner),
                properties: Vec::new(),
                skin: Default::default(),
            };
            let mut head = horizon_client::items::ItemStack::new("minecraft:player_head", 1);
            let tag = synced
                .as_ref()
                .and_then(|t| t.profile.clone())
                .unwrap_or_else(|| horizon_client::item_sync::profile_tag(&profile));
            head.data_mut().set_other("minecraft:profile", Some(tag));
            let size = k::BUBBLE_ICON * px;
            draw.translate([0.0, 0.0, 0.02]);
            draw.scale([size, size, size * 0.02]);
            let item = game.resolve_item(&head, horizon_assets::item_model::DisplayContext::Gui, None, 0);
            if !item.is_empty() {
                let pose = draw.pose();
                game.submit_item_unlit(&item, pose, full_bright, draw.vertices_mut());
            }
        }
        k::Hint::Tamed => {
            let (hw, hh) = (k::HEART[0].len() as f32, k::HEART.len() as f32);
            pixels(k::heart_runs(), [-hw / 2.0, -hh / 2.0], 0.01);
        }
        k::Hint::Items(items) => {
            // Flattened onto the bubble, just in front of it: a block item
            // keeps its inventory look without sticking out.
            let size = k::BUBBLE_ICON * px;
            draw.translate([0.0, 0.0, 0.02]);
            draw.scale([size, size, size * 0.02]);
            // Unlit, as in the inventory: the world's side shading would grey
            // it (sugar looked like gunpowder).
            let stack =
                horizon_client::items::ItemStack::new(&k::hint_item(items, game.world.game_time), 1);
            let item = game.resolve_item(&stack, horizon_assets::item_model::DisplayContext::Gui, None, 0);
            if !item.is_empty() {
                let pose = draw.pose();
                game.submit_item_unlit(&item, pose, full_bright, draw.vertices_mut());
            }
        }
    }
    draw.pop();
}
