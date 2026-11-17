// `getLeashOffset`: a lead on a mob that can be tamed hangs from the middle of
// its body (its centre), not from its head.
fn mord_tame_lead_offset(m: &crate::mobs::Mob) -> core::ops::ControlFlow<[f64; 3]> {
    if crate::mobs::mord::lead_at_middle(&m.kind.id) {
        core::ops::ControlFlow::Break([0.0, f64::from(m.height()) * 0.5, 0.0])
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
