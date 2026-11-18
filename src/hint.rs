//! Taming hints: over the tameable mob the player looks at (the nearest along
//! the view, within `HINT_DISTANCE`, no block between), a thought bubble
//! facing the camera with the item that tames it, a heart once it is tamed,
//! or the head of the player who tamed it when that is someone else. Drawn on
//! the client, after the mob (`render_living_post`).

use std::time::Instant;

use crate::logic as k;
use crate::tamed::{self, TAMED};
use mod_api::{
    AttachmentHolder, AttachmentQueries, BlockEntityDrawContext, ClientHost, Compound, EntityRef,
    Host, Identifier, ItemDisplayContext, ItemStack, RenderLivingPostEvent, Tag,
};

/// The white sprite the bubble's pixels are drawn with, tinted
/// (`assets/kidmode/textures/block/white.png`).
const WHITE: &str = "kidmode:block/white";

/// Full sky and block light, so the bubble reads at night too.
const FULL_BRIGHT: u32 = 240 | 240 << 8;

/// Draws the bubble over `event`'s mob when the player looks at it, and lets
/// it grow in and shrink away.
pub fn render(host: &dyn ClientHost, event: &mut RenderLivingPostEvent<'_>) {
    let mob = event.entity;
    if !k::may_hint(mob.kind) {
        return;
    }
    let Some(entity) = host.entity(mob.id) else {
        return;
    };
    let Some(camera) = host.camera() else {
        return;
    };
    let Some(bounds) = entity.bounding_box(host) else {
        return;
    };
    let synced = synced(host, entity);
    let own = host.local_player().map(|p| p.uuid());
    let living = host.living(entity);
    let state = k::TameState {
        tame: living.and_then(|l| l.is_tamed(host)).unwrap_or(false),
        // Not readable on the client through the API (docs/missing.md).
        trusting: false,
        angry: false,
        baby: mob.baby,
        synced_tamed: synced.is_some(),
        tamed_by_other: synced
            .as_ref()
            .and_then(|t| t.owner)
            .is_some_and(|owner| Some(owner) != own),
    };
    let Some(hint) = k::hint(mob.kind, mob.id, state) else {
        return;
    };

    // Looked at: hit along the view within reach, with no block in between.
    let eye = camera.position;
    let dir = k::view_direction(camera.yaw, camera.pitch);
    let distance = k::ray_box(eye, dir, bounds, k::HINT_DISTANCE).filter(|&d| {
        host.raycast(d as f32)
            .is_none_or(|hit| hit_distance(eye, &hit) >= d)
    });
    let frame = event.draw.game_time() as u64 * 1000 + (camera.partial_tick * 999.0) as u64;
    let looked = k::looked_at(frame, mob.id, distance);
    let shown = k::hint_alpha(mob.id, looked, Instant::now());
    if shown <= 0.0 {
        return;
    }

    let height = (bounds[1][1] - bounds[0][1]) as f32;
    let lift = k::bubble_lift(height, mob.custom_name.is_some());
    let feet = entity.position(host).unwrap_or(eye);
    let draw = &mut *event.draw;
    draw.push();
    draw.set_light(FULL_BRIGHT);
    // Back from the body's turn to the world's axes, up to the bubble's
    // middle, then turned to face the camera: +x its right, +y up, +z towards
    // the camera.
    draw.rotate_about([0.0, 1.0, 0.0], mob.body_yaw, [0.0; 3]);
    let px = k::BUBBLE_PIXEL;
    let [cx, cy] = k::BUBBLE_ICON_CENTRE;
    let middle = lift + (k::BUBBLE_H as f32 - cy) * px;
    draw.translate([0.0, middle, 0.0]);
    let to_camera = [
        (eye[0] - feet[0]) as f32,
        (eye[1] - feet[1]) as f32 - middle,
        (eye[2] - feet[2]) as f32,
    ];
    let length = to_camera.iter().map(|c| c * c).sum::<f32>().sqrt();
    if length > 1.0e-3 {
        let yaw = to_camera[0].atan2(to_camera[2]).to_degrees();
        let pitch = (to_camera[1] / length).clamp(-1.0, 1.0).asin().to_degrees();
        draw.rotate_about([0.0, 1.0, 0.0], yaw, [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], -pitch, [0.0; 3]);
    }
    // It grows from its tail as it appears, and shrinks back as it goes.
    let s = shown.clamp(0.0, 1.0);
    let tail = -(k::BUBBLE_H as f32 - cy) * px;
    draw.translate([0.0, tail, 0.0]);
    draw.scale([s, s, s]);
    draw.translate([0.0, -tail, 0.0]);

    pixels(draw, k::bubble_runs(), [-cx, -cy], 0.0);
    match hint {
        k::Hint::Tamed => {
            let (hw, hh) = (k::HEART[0].len() as f32, k::HEART.len() as f32);
            pixels(draw, k::heart_runs(), [-hw / 2.0, -hh / 2.0], 0.01);
        }
        k::Hint::TamedBy => {
            if let Some(head) = synced.as_ref().and_then(|t| head(host, t)) {
                icon(draw, &head);
            }
        }
        k::Hint::Items(items) => {
            let item = k::hint_item(items, draw.game_time());
            if let Some(stack) = stack_of(host, &item) {
                icon(draw, &stack);
            }
        }
    }
    draw.pop();
}

/// What the server synced about the mob's taming (`kidmode:tamed`).
fn synced(host: &dyn ClientHost, entity: EntityRef) -> Option<tamed::Tamed> {
    host.query(&TAMED)
        .ok()?
        .get(AttachmentHolder::Entity(entity))
        .ok()
        .flatten()
}

/// How far from `eye` the raycast's block hit is.
fn hit_distance(eye: [f64; 3], hit: &mod_api::Hit) -> f64 {
    let d = [hit.x - eye[0], hit.y - eye[1], hit.z - eye[2]];
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}

/// A picture as runs of pixels (row, first column, column after the last,
/// 0xRRGGBB), `at` its top left from the icon's centre, `z` towards the
/// camera, in bubble pixels.
fn pixels(
    draw: &mut dyn BlockEntityDrawContext,
    runs: Vec<(usize, usize, usize, u32)>,
    at: [f32; 2],
    z: f32,
) {
    let px = k::BUBBLE_PIXEL;
    for (row, x0, x1, color) in runs {
        let (l, r) = ((at[0] + x0 as f32) * px, (at[0] + x1 as f32) * px);
        let (t, b) = (-(at[1] + row as f32) * px, -(at[1] + row as f32 + 1.0) * px);
        // A model face's order: top left, bottom left, bottom right, top right.
        let corners = [[l, t, z], [l, b, z], [r, b, z], [r, t, z]];
        draw.quad(WHITE, corners, [0.0, 0.0, 16.0, 16.0], color);
    }
}

/// An item flattened onto the bubble, just in front of it, so a block item
/// keeps its inventory look without sticking out.
fn icon(draw: &mut dyn BlockEntityDrawContext, stack: &ItemStack) {
    let size = k::BUBBLE_ICON * k::BUBBLE_PIXEL;
    draw.push();
    draw.translate([0.0, 0.0, 0.02]);
    draw.scale([size, size, size * 0.02]);
    draw.item(stack, ItemDisplayContext::Gui, None, 0);
    draw.pop();
}

fn stack_of(host: &dyn Host, id: &str) -> Option<ItemStack> {
    let key = host.item_key(&Identifier::parse_namespaced(id)?)?;
    Some(ItemStack::of(key))
}

/// The tamer's head, with the profile (and skin) the server synced, or their
/// id and name.
fn head(host: &dyn Host, tamed: &tamed::Tamed) -> Option<ItemStack> {
    let mut head = stack_of(host, "minecraft:player_head")?;
    let profile = tamed.profile.clone().or_else(|| {
        let mut p: Compound = Compound::new();
        if let Some(owner) = tamed.owner {
            p.insert("id".into(), Tag::IntArray(tamed::uuid_ints(owner)));
        }
        if let Some(name) = &tamed.owner_name {
            p.insert("name".into(), Tag::String(name.as_str().into()));
        }
        (!p.is_empty()).then_some(Tag::Compound(p))
    })?;
    head.set_components([("minecraft:profile".into(), profile)].into())
        .ok()?;
    Some(head)
}
