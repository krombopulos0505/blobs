use crate::blob::action::Action;
use crate::blob::Blob;

pub struct Decision {
    pub blob_idx: usize,
    pub kind: Action,
    pub strength: f32,
}

pub fn decide_phase(blobs: &[Blob]) -> Vec<Decision> {
    blobs
        .iter()
        .enumerate()
        .map(|(i, blob)| {
            let (action, strength) = blob.grn.output();
            Decision { blob_idx: i, kind: action, strength }
        })
        .collect()
}
