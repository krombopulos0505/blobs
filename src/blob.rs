pub mod genome;

use genome::Genome;
use crate::position::{Pos, Direction};
use crate::simulation::Simulation;

pub struct Blob {
    pub pos: Pos,
    pub genome: Genome,
}

impl Blob {
    pub fn sense(&mut self, sim: &Simulation) {
        self.genome.proteins[0] = (self.energy as f32 - BASE_ENERGY) / BASE_ENERGY;
        self.genome.proteins[1] = self.see_blob(sim);
        self.genome.proteins[2] = self.see_food(sim);
        self.genome.proteins[4] = sim.world.brightness();
        self.genome.proteins[5] = (2 * self.x - sim.world.width) as f32 / sim.world.width as f32;
        self.genome.proteins[5] = (2 * self.y - sim.world.height) as f32 / sim.world.height as f32;
        self.genome.proteins[7] = sim.rng.gen_range(-1.0..1.0);
    }

    pub fn act(&mut self, action: (usize, f32), sim: &mut Simulation) {
        match action.0 {
            0 => self.walk(sim),
            1 => {
                self.dir = sim.rng.gen_range(0..8);
                self.walk(sim);
            }
            2 => self.dir = (self.dir + 1) % 8,
            3 => self.eat_food(sim),
            4 => self.photosyn(sim),
            5 => self.replicate(sim),
            6 => self.attack(sim),
            _ => {}
        }
    }

    pub fn see_blob(&self, sim: &Simulation) -> f32 {
        let npos = self.pos.neighbor(self.dir);
        if npos.in_bounds(&sim.world) {
            if matches!(sim.world.get_tile(&npos), Tile::Blob(idx)) {
                return 1.0;
            }
        }
        -1.0
    }

    pub fn see_food(&self, sim: &Simulation) -> f32 {
        let npos = self.pos.neighbor(self.dir);
        if npos.in_bounds(&sim.world) {
            if matches!(sim.world.get_tile(&npos), Tile::Food) {
                return 1.0;
            }
        }
        -1.0
    }

    pub fn walk(&mut self, sim: &Simulation) {
        let npos = self.pos.neighbor(self.dir);
        if npos.in_bounds(&sim.world) {
            if matches!(sim.world.get_tile(&npos), Tile::Empty) {
                self.pos = npos;
            }
        }
    }

    pub fn eat_food(&mut self, sim: &mut Simulation) {
        if matches!(sim.world.get_tile(&self.pos), Tile::Food) {
            sim.world.set_tile(&self.pos, Tile::NoFood);
            self.energy += FOOD_ENERGY_GAIN;
        }
    }

    pub fn photosyn(&mut self, sim: &Simulation) {
        if sim.world.brightness() > 0.0 {
            self.energy += (sim.world.brightness() * PHOT_ENERGY_GAIN) as i16;
        }
    }

    pub fn replicate(&mut self, sim: &mut Simulation) {
        if let Some(pos) = self.pos.find_empty_neighbor(&sim.world) {
            sim.blobs.push(Blob {
                pos: pos,
                dir: 0,
                energy: self.energy / 2,
                genome: Genome::mutate(&self.genome),
            });
            self.energy /= 2;
        }
    }

    pub fn attack(&mut self, sim: &mut Simulation) {
        let npos = self.pos.neighbor(self.dir);
        if npos.in_bounds(&sim.world) {
            match sim.world.get_tile(&npos) {
                Tile::Blob(idx) => {
                    let damage = sim.rng.gen_range(0..BASE_ENERGY/2) - sim.blobs[idx];
                    sim.blobs[idx].energy -= damage;
                    self.energy += damage;
                _ => {}
            }
        }
    }
}
