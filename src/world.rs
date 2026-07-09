use rand::{Rng, rngs::ThreadRng};
use crate::position::Pos;

pub struct Tile {
    pub blob: Option<u64>,
    pub food: bool,
    pub wall: bool,
}

#[derive(Default)]
pub struct World {
    pub width: i16,
    pub height: i16,
    pub tiles: Vec<Tile>,
    pub tick: u64,
    pub last_id: u64,
}

impl World {
    pub fn new(width: i16, height: i16) -> Self {
        Self {
            width: width,
            height: height,
            tiles: Vec::with_capacity((width * height) as usize),
            tick: 0,
            last_id: 0,
        }
    }

    pub fn step(&mut self, rng: &mut ThreadRng) {
        self.tick += 1;

        self.set_food(
            &Pos::new(rng.gen_range(0..self.width), rng.gen_range(0..self.height)),
            true
        );
    }

    pub fn get_blob(&self, pos: &Pos) -> Option<u64> {
        let id = (pos.x + pos.y * self.width) as usize;
        self.tiles[id].blob
    }

    pub fn set_blob(&mut self, pos: &Pos, blob: Option<u64>) {
        let id = (pos.x + pos.y * self.width) as usize;
        self.tiles[id].blob = blob;
    }

    pub fn get_food(&self, pos: &Pos) -> bool {
        let id = (pos.x + pos.y * self.width) as usize;
        self.tiles[id].food
    }

    pub fn set_food(&mut self, pos: &Pos, val: bool) {
        let id = (pos.x + pos.y * self.width) as usize;
        self.tiles[id].food = val;
    }

    pub fn get_wall(&self, pos: &Pos) -> bool {
        let id = (pos.x + pos.y * self.width) as usize;
        self.tiles[id].wall
    }

    pub fn set_wall(&mut self, pos: &Pos, val: bool) {
        let id = (pos.x + pos.y * self.width) as usize;
        self.tiles[id].wall = val;
    }

    pub fn brightness(&self) -> f32 {
        if (3..9).contains(&self.tick) {
            return 0.5;
        } else if (9..15).contains(&self.tick) {
            return 1.0;
        } else if (15..21).contains(&self.tick) {
            return 0.5;
        } else {
            return 0.0
        }
    }
}
