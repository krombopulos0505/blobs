use super::context::SimulationContext;

pub fn think_phase(ctx: &mut SimulationContext) {
    for blob in ctx.blobs.iter_mut() {
        blob.grn.step(&blob.genome);
    }
}
