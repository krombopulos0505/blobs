use rand::{Rng, ThreadRng};
use crate::world::World;
use crate::blob::Blob;

pub struct Simulation {
    pub world: World,
    pub blobs: Vec<Blob>,
    pub rng: ThreadRng,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            blobs: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn step(&mut self) {
        self.world.step();

        for blob in &mut self.blobs {
            blob.sense(&self.world, &mut self.rng);
        }

        for blob in &mut self.blobs {
            blob.genome.step();
        }

        for blob in &mut self.blobs {
            blob.act(blob.genome.output(), &mut self.world, 
                &mut self.blobs, &mut self.rng);
        }
    }
}
