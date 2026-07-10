pub mod action;
pub mod genome;
pub mod grn;
pub mod sensor;

use rand::Rng;

use crate::balance::BASE_ENERGY;
use crate::position::{Dir, Pos};
use crate::world::World;
use genome::Genome;
use grn::GRN;

pub type BlobId = u64;

pub struct Blob {
    pub id: BlobId,
    pub pos: Pos,
    pub dir: Dir,
    pub energy: i16,
    pub def: i16,
    pub genome: Genome,
    pub grn: GRN,
}

impl Blob {
    pub fn minimal_viable(world: &mut World, rng: &mut impl Rng) -> Self {
        let pos = Pos::new(
            rng.gen_range(0..world.width()),
            rng.gen_range(0..world.height()),
        );
        let id = world.next_id();

        let blob = Self {
            id,
            pos,
            dir: Dir::N,
            energy: BASE_ENERGY,
            def: 0,
            genome: Genome::minimal_viable(rng),
            grn: GRN::default(),
        };

        world.occupant.set(blob.pos, Some(blob.id));
        blob
    }

    pub fn is_dead(&self) -> bool {
        self.energy <= 0
    }
}
