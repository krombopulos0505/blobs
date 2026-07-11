mod apply;
mod command;
mod context;
mod decide;
mod resolve;
mod sense;
mod think;

use rand::rngs::ThreadRng;

use crate::balance::{WORLD_HEIGHT, WORLD_WIDTH};
use crate::blob::Blob;
use crate::world::World;
use apply::apply_phase;
use decide::decide_phase;
use resolve::resolve_phase;
use sense::sense_phase;
use think::think_phase;

pub use context::SimulationContext;

pub struct Simulation {
    pub world: World,
    pub blobs: Vec<Blob>,
    pub rng: ThreadRng,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            world: World::new(WORLD_WIDTH, WORLD_HEIGHT),
            blobs: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn step(&mut self) {
        let mut ctx = SimulationContext {
            world: &mut self.world,
            blobs: &mut self.blobs,
            rng: &mut self.rng,
        };

        sense_phase(&mut ctx);
        think_phase(&mut ctx);

        let decisions = decide_phase(&ctx);
        let cmds = apply_phase(&mut ctx, &decisions);
        resolve_phase(&mut ctx, cmds);

        ctx.world.advance_tick(ctx.rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blob::Blob;

    /// Regression guard for the pipeline plumbing itself: seeds a
    /// handful of blobs and runs several thousand ticks. This doesn't
    /// assert anything about population dynamics (that's a balance
    /// question, not a correctness one) -- it exists to catch panics
    /// from stale indices, grid desyncs, or bad borrows in the command
    /// buffer.
    #[test]
    fn many_ticks_do_not_panic() {
        let mut sim = Simulation::new();
        for _ in 0..8 {
            let blob = Blob::minimal_viable(&mut sim.world, &mut sim.rng);
            sim.blobs.push(blob);
        }

        for _ in 0..20_000 {
            sim.step();
            if sim.blobs.is_empty() {
                break; // extinction is a valid outcome, not a bug
            }
        }
    }
}
