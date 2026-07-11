use rand::rngs::ThreadRng;

use crate::blob::Blob;
use crate::world::World;

/// Everything a pipeline phase might need, bundled behind one parameter
/// instead of a `world`/`blobs`/`rng` triple threaded through every
/// function signature.
///
/// Phases take `&SimulationContext` or `&mut SimulationContext` and
/// access fields directly (`ctx.blobs`, `ctx.world`, `ctx.rng`) rather
/// than through methods on `SimulationContext` itself. That keeps the
/// fields independently borrowable: `sense_phase` mutates `blobs` while
/// reading `world` and mutating `rng` in the same call, which wouldn't
/// borrow-check if this were one opaque method call taking `&mut self`.
///
/// This is also the shape an ECS migration slots into later: `world` as
/// resources, `blobs` as a component query, `rng` as a resource -- a
/// system function taking `&mut SimulationContext` looks a lot like a
/// system taking a `Query` + `Res` set.
pub struct SimulationContext<'a> {
    pub world: &'a mut World,
    pub blobs: &'a mut Vec<Blob>,
    pub rng: &'a mut ThreadRng,
}
