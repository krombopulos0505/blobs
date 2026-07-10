use crate::position::Pos;

/// A flat 2D grid of `T`. Every world-state layer (occupant, food, wall, ...)
/// is one of these, so index math only ever lives here.
#[derive(Clone)]
pub struct Grid<T> {
    w: i16,
    h: i16,
    cells: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(w: i16, h: i16) -> Self {
        Self {
            w,
            h,
            cells: vec![T::default(); (w * h) as usize],
        }
    }

    pub fn width(&self) -> i16 {
        self.w
    }

    pub fn height(&self) -> i16 {
        self.h
    }

    fn idx(&self, p: Pos) -> usize {
        (p.x + p.y * self.w) as usize
    }

    pub fn get(&self, p: Pos) -> &T {
        &self.cells[self.idx(p)]
    }

    pub fn get_mut(&mut self, p: Pos) -> &mut T {
        let i = self.idx(p);
        &mut self.cells[i]
    }

    pub fn set(&mut self, p: Pos, val: T) {
        let i = self.idx(p);
        self.cells[i] = val;
    }

    pub fn in_bounds(&self, p: Pos) -> bool {
        (0..self.w).contains(&p.x) && (0..self.h).contains(&p.y)
    }
}
