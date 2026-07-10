use rand::Rng;

use crate::balance::DAY_LENGTH;
use crate::blob::BlobId;
use crate::grid::Grid;
use crate::position::Pos;

pub struct World {
    pub occupant: Grid<Option<BlobId>>,
    pub food: Grid<bool>,
    pub wall: Grid<bool>,
    pub tick: u64,
    last_id: BlobId,
}

impl World {
    pub fn new(w: i16, h: i16) -> Self {
        Self {
            occupant: Grid::new(w, h),
            food: Grid::new(w, h),
            wall: Grid::new(w, h),
            tick: 0,
            last_id: 0,
        }
    }

    pub fn width(&self) -> i16 {
        self.occupant.width()
    }

    pub fn height(&self) -> i16 {
        self.occupant.height()
    }

    pub fn in_bounds(&self, p: Pos) -> bool {
        self.occupant.in_bounds(p)
    }

    pub fn next_id(&mut self) -> BlobId {
        let id = self.last_id;
        self.last_id += 1;
        id
    }

    pub fn advance_tick(&mut self, rng: &mut impl Rng) {
        self.tick += 1;

        // sprinkle in one piece of food per tick; tune this once you're
        // tracking population/food ratios instead of guessing
        let spot = Pos::new(rng.gen_range(0..self.width()), rng.gen_range(0..self.height()));
        self.food.set(spot, true);
    }

    /// 0.0 (night) .. 1.0 (full daylight), cycling every DAY_LENGTH ticks
    pub fn brightness(&self) -> f32 {
        let t = self.tick % DAY_LENGTH;
        if (3..9).contains(&t) {
            0.5
        } else if (9..15).contains(&t) {
            1.0
        } else if (15..21).contains(&t) {
            0.5
        } else {
            0.0
        }
    }
}
