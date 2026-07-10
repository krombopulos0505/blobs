mod apply;
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
            rng: rand::thread_rng() 
        }
    }

    pub fn step(&mut self) {
        sense_phase(&mut self.blobs, &self.world, &mut self.rng);
        think_phase(&mut self.blobs);

        let decisions = decide_phase(&self.blobs);
        let cmds = apply_phase(&self.blobs, &self.world, &decisions, &mut self.rng);
        resolve_phase(&mut self.blobs, &mut self.world, cmds);

        self.world.advance_tick(&mut self.rng);
    }
}
