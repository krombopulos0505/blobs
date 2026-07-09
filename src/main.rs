mod position;
mod blob;
mod world;
mod simulation;

use position::Pos;
use blob::{
    Blob, 
    genome::{Genome, Gene},
};

fn main() {
    let mut blob = Blob {
        pos: Pos::new(0, 0),
        genome: Genome::default(),
    };

    blob.genome.genes.push(Gene {
        src: 0,
        tgt: 9,
        weight: 1.0,
        threshold: 0.0,
    });

    blob.step();
}
