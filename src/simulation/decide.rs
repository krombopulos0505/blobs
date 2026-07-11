use super::context::SimulationContext;
use crate::blob::action::Action;

pub struct Decision {
    pub blob_idx: usize,
    pub kind: Action,
    pub strength: f32,
}

pub fn decide_phase(ctx: &SimulationContext) -> Vec<Decision> {
    ctx.blobs
        .iter()
        .enumerate()
        .map(|(i, blob)| {
            let (action, strength) = blob.grn.output();
            Decision {
                blob_idx: i,
                kind: action,
                strength,
            }
        })
        .collect()
}
