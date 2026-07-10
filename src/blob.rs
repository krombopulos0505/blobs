pub mod genome;

use rand::{Rng, rngs::ThreadRng};
use genome::Genome;
use crate::position::Pos;
use crate::world::World;
use crate::balance::*;

pub struct Blob {
    pub id: u64,
    pub pos: Pos,
    pub dir: usize,
    pub energy: i16,
    pub def: i16,
    pub genome: Genome,
}

impl Blob {
    pub fn minimal_viable(world: &mut World, rng: &mut ThreadRng) -> Self {
        let blob = Self {
            id: world.last_id,
            pos: Pos::new(
                rng.gen_range(0..world.width), 
                rng.gen_range(0..world.height)
            ),
            dir: 0,
            energy: BASE_ENERGY,
            def: 0,
            genome: Genome::minimal_viable(rng),
        };
        world.last_id += 1;
        world.set_blob(&blob.pos, Some(blob.id));
        blob
    }

    pub fn sense(&mut self, world: &World, rng: &mut ThreadRng) {
        self.genome.proteins[0] = (self.energy - BASE_ENERGY) as f32 / 
            BASE_ENERGY as f32;
        self.genome.proteins[1] = self.see_blob(world);
        self.genome.proteins[2] = self.see_food(world);
        self.genome.proteins[4] = world.brightness();
        self.genome.proteins[5] = (2 * self.pos.x - world.width) as f32 / 
            world.width as f32;
        self.genome.proteins[6] = (2 * self.pos.y - world.height) as f32 / 
            world.height as f32;
        self.genome.proteins[7] = rng.gen_range(-1.0..1.0);
    }

    pub fn act(&mut self, action: (usize, f32), world: &mut World, 
        blobs: &mut Vec<Blob>, rng: &mut ThreadRng) -> bool {
        match action.0 {
            0 => self.walk(world),
            1 => {
                self.dir = rng.gen_range(0..8);
                self.walk(world);
            }
            2 => self.dir = (self.dir + 1) % 8,
            3 => self.eat_food(world),
            4 => self.photosyn(world),
            5 => self.replicate(world, blobs, rng),
            6 => self.attack(world, blobs, rng),
            _ => {}
        }

        self.energy -= METABOLISM;
        self.is_dead()
    }

    pub fn see_blob(&self, world: &World) -> f32 {
        let npos = self.pos.neighbor(self.dir);
        if npos.in_bounds(world) {
            if world.get_blob(&npos).is_some() {
                return 1.0;
            }
        }
        -1.0
    }

    pub fn see_food(&self, world: &World) -> f32 {
        let npos = self.pos.neighbor(self.dir);
        if npos.in_bounds(world) {
            if world.get_food(&npos) {
                return 1.0;
            }
        }
        -1.0
    }

    pub fn walk(&mut self, world: &mut World) {
        let npos = self.pos.neighbor(self.dir);
        if npos.in_bounds(world) {
            if world.get_blob(&npos).is_none() &&
                !world.get_wall(&npos) {
                    world.set_blob(&self.pos, None);
                    self.pos = npos;
                    world.set_blob(&self.pos, Some(self.id));
                    self.energy -= ACTION_COST;
            }
        }
    }

    pub fn eat_food(&mut self, world: &mut World) {
        if world.get_food(&self.pos) {
            world.set_food(&self.pos, false);
            self.energy += FOOD_ENERGY_GAIN;
            self.energy -= ACTION_COST;
        }
    }

    pub fn photosyn(&mut self, world: &mut World) {
        self.energy += (world.brightness() * PHOT_ENERGY_GAIN) as i16;
        self.energy -= ACTION_COST;
    }

    pub fn replicate(&mut self, world: &mut World,
        blobs: &mut Vec<Blob>, rng: &mut ThreadRng) {
        if let Some(pos) = self.pos.find_empty_neighbor(world) {
            blobs.push(Blob {
                id: world.last_id,
                pos: pos,
                dir: 0,
                energy: self.energy / 2,
                def: self.def,
                genome: Genome::mutate(&self.genome, rng),
            });
            world.last_id += 1;
            self.energy /= 2;
            self.energy -= REPLICATION_COST;
        }
    }

    pub fn attack(&mut self, world: &mut World,
        blobs: &mut Vec<Blob>, rng: &mut ThreadRng) {
        let npos = self.pos.neighbor(self.dir);
        if npos.in_bounds(world) {
            if let Some(id) = world.get_blob(&npos) {
                blobs.iter_mut()
                    .filter(|blob| blob.id == id)
                    .for_each(|blob| {
                        let damage = rng.gen_range(0..BASE_ENERGY/2) - 
                            blob.def;
                        blob.energy -= damage;
                    });
                self.energy -= ATTACK_COST;
            }
        }
    }

    pub fn is_dead(&self) -> bool {
        self.energy <= 0
    }
}
