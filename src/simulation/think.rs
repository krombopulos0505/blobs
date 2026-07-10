use crate::blob::Blob;

pub fn think_phase(blobs: &mut [Blob]) {
    for blob in blobs.iter_mut() {
        blob.grn.step(&blob.genome);
    }
}
