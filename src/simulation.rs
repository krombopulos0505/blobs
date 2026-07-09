use rand::{Rng, ThreadRng};
use crate::world::World;
use crate::cell::Cell;

pub struct Simulation {
    pub world: World,
    pub cells: Vec<Cell>,
    pub rng: ThreadRng,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            cells: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn step(&mut self) {
        let old_sim = self;

        for cell in self.cells.iter_mut() {
            cell.step(&mut old_sim);
        }

        self.foods = old_sim.foods;
    }
}
