mod position;
mod blob;
mod world;
mod simulation;
mod balance;

use blob::Blob;
use simulation::Simulation;

fn main() {
    let mut sim = Simulation::new();
    sim.blobs.push(Blob::minimal_viable(&mut sim.world, &mut sim.rng));
    
    sim.step();
}
