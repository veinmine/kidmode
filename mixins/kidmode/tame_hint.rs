// `native_mob_layer`: over the tameable, untamed mob the player looks at
// (nearest along the view, within `HINT_DISTANCE`, no block between), a
// thought bubble facing the camera with the item that tames it.
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
    let state = k::TameState {
        tame: m.extra.tame,
        trusting: m.extra.trusting,
        angry: m.extra.anger > 0,
        baby: m.is_baby(),
        guest: game.remote.is_some(),
        named: mob.custom_name.is_some(),
    };
    let Some(items) = k::hint_items(mob.kind, mob.id, state) else { return };
    let Some((eye, dir)) = game.aim() else { return };
    let Some((index, distance, _)) = game.mob_hit_within(k::HINT_DISTANCE) else { return };
    if game.mobs[index].id != mob.id || game.clip_ray(eye, dir, distance).is_some() {
        return;
    }
    let bounds = m.bounds();
    let height = (bounds.max_position().y - bounds.min_position().y) as f32;
    let px = k::BUBBLE_PIXEL;
    let (w, h) = (k::BUBBLE_W as f32, k::BUBBLE_H as f32);
    let full_bright = 240 | 240 << 8;
    draw.push();
    // Back from the body's turn to the world's axes, up over the head, then
    // facing the camera: +x its right, +y up, +z towards it.
    draw.rotate_about([0.0, 1.0, 0.0], mob.body_yaw, [0.0; 3]);
    draw.translate([0.0, k::bubble_lift(height, mob.custom_name.is_some()), 0.0]);
    draw.rotate_about([0.0, 1.0, 0.0], 180.0 - game.yaw, [0.0; 3]);
    draw.rotate_about([1.0, 0.0, 0.0], -game.pitch, [0.0; 3]);
    let white = game.gui.atlas.font.white();
    let uv = [(white.u0 + white.u1) / 2.0, (white.v0 + white.v1) / 2.0];
    let pose = draw.pose();
    for (row, x0, x1, color) in k::bubble_runs() {
        let (l, r) = ((x0 as f32 - w / 2.0) * px, (x1 as f32 - w / 2.0) * px);
        let (t, b) = ((h - row as f32) * px, (h - row as f32 - 1.0) * px);
        // A model face's order: top left, bottom left, bottom right, top right.
        for [x, y] in [[l, t], [l, b], [r, b], [r, t]] {
            let at = pose.transform_point3(glam::Vec3::new(x, y, 0.0));
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
    let [cx, cy] = k::BUBBLE_ICON_CENTRE;
    let size = k::BUBBLE_ICON * px;
    draw.translate([(cx - w / 2.0) * px, (h - cy) * px, size * 0.4]);
    draw.scale([size, size, size]);
    let light = draw.light;
    draw.light = full_bright;
    let stack = horizon_client::items::ItemStack::new(&k::hint_item(items, game.world.game_time), 1);
    game.native_draw_item(draw, &stack, horizon_assets::item_model::DisplayContext::Gui, None, 0);
    draw.light = light;
    draw.pop();
}
