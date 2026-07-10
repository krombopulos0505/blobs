use rand::rngs::ThreadRng;
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
            world: World::new(80, 24),
            blobs: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn step(&mut self) {
        self.world.step(&mut self.rng);

        for blob in &mut self.blobs {
            blob.sense(&self.world, &mut self.rng);
        }

        for blob in &mut self.blobs {
            blob.genome.step();
        }

        for i in 0..self.blobs.len() {
            let mut blob = self.blobs.remove(i);

            let action = blob.genome.output();
            let is_dead = blob.act(action, &mut self.world, 
                &mut self.blobs, &mut self.rng);

            if !is_dead {
                self.blobs.insert(i, blob);
            } else {
                self.world.set_blob(&blob.pos, None);
                self.world.set_food(&blob.pos, true);
            }
        }
    }
}
