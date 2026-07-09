use crate::position::Pos;

pub enum Tile {
    Empty,
    Food, NoFood,
    Blob(usize), NoBlob,
}

#[derive(Default)]
pub struct World {
    pub width: i16,
    pub height: i16,
    pub tiles: Vec<Tile>,
}

impl World {
    pub fn get_tile(&self, pos: &Pos) -> Tile {
        self.tiles[pos.x + pos.y * self.width] 
    }

    pub fn get_tile(&mut self, pos: &Pos, tile: Tile) {
        self.tiles[pos.x + pos.y * self.width] = tile;
    }
}
