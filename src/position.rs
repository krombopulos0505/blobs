use std::ops::{Add, Sub};
use crate::world::World;

#[derive(Copy, Clone)]
pub struct Pos {
    pub x: i16,
    pub y: i16,
}

impl Pos {
    const NEIGHBORS: [Self; 8] = [
        Self { x: -1, y: -1 }, Self { x: 0, y: -1 },
        Self { x: 1, y: -1 }, Self { x: 1, y: 0 },
        Self { x: 1, y: 1}, Self { x: 0, y: 1 },
        Self { x: -1, y: 1 }, Self { x: -1, y: 0},
    ];

    pub fn new(x: i16, y: i16) -> Self {
        Self { x: x, y: y }
    }
    
    pub fn in_bounds(&self, world: &World) -> bool {
        (0..world.width).contains(&self.x) &&
            (0..world.height).contains(&self.y)
    }

    pub fn neighbor(&self, dir: usize) -> Self {
        *self + Self::NEIGHBORS[dir]
    }

    pub fn find_empty_neighbor(&self, world: &World) -> Option<Self> {
        for dir in 0..8 {
            let npos = self.neighbor(dir);
            if world.get_blob(&npos).is_none() {
                return Some(npos)
            }
        }
        None
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
