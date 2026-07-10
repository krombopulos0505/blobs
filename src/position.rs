use rand::Rng;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Dir { N, NE, E, SE, S, SW, W, NW }

impl Dir {
    pub const ALL: [Self; 8] = [
        Dir::N,
        Dir::NE,
        Dir::E,
        Dir::SE,
        Dir::S,
        Dir::SW,
        Dir::W,
        Dir::NW,
    ];

    pub fn offset(self) -> (i16, i16) {
        match self {
            Dir::N => (0, -1),
            Dir::NE => (1, -1),
            Dir::E => (1, 0),
            Dir::SE => (1, 1),
            Dir::S => (0, 1),
            Dir::SW => (-1, 1),
            Dir::W => (-1, 0),
            Dir::NW => (-1, -1),
        }
    }

    /// rotate one step clockwise
    pub fn cw(self) -> Self {
        Self::ALL[(self as usize + 1) % 8]
    }

    pub fn from_index(i: usize) -> Self {
        Self::ALL[i % 8]
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        Self::ALL[rng.gen_range(0..8)]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pos {
    pub x: i16,
    pub y: i16,
}

impl Pos {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn neighbor(&self, dir: Dir) -> Self {
        let (dx, dy) = dir.offset();
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
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
