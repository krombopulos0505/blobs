use crate::blob::genome::Genome;
use crate::blob::{Blob, BlobId};
use crate::position::{Dir, Pos};
use crate::world::World;

pub enum Command {
    Move { blob_idx: usize, to: Pos },
    Turn { blob_idx: usize, dir: Dir },
    Eat { blob_idx: usize, at: Pos },
    Photosynthesize { blob_idx: usize, gain: i16 },
    Spawn { genome: Genome, at: Pos, energy: i16 },
    Damage { target_id: BlobId, amount: i16 },
    SpendEnergy { blob_idx: usize, amount: i16 },
}

pub fn resolve_phase(blobs: &mut Vec<Blob>, world: &mut World, cmds: Vec<Command>) {
    let mut spawns = Vec::new();

    // Commands were computed against a snapshot of the world taken before
    // this tick's moves/spawns happened. Two blobs can both have decided
    // "that tile is empty" against the same snapshot. So every command that
    // claims a tile gets re-checked HERE, at commit time, against the
    // world as it stands after everything processed so far this tick —
    // first command to reach a tile wins, later ones are simply dropped.
    for cmd in cmds {
        match cmd {
            Command::Move { blob_idx, to } => {
                if world.occupant.get(to).is_none() {
                    move_blob(&mut blobs[blob_idx], to, world);
                }
            }
            Command::Turn { blob_idx, dir } => blobs[blob_idx].dir = dir,
            Command::Eat { blob_idx, at } => eat(&mut blobs[blob_idx], at, world),
            Command::Photosynthesize { blob_idx, gain } => blobs[blob_idx].energy += gain,
            Command::SpendEnergy { blob_idx, amount } => blobs[blob_idx].energy -= amount,
            Command::Damage { target_id, amount } => {
                if let Some(t) = blobs.iter_mut().find(|b| b.id == target_id) {
                    t.energy -= amount;
                }
            }
            Command::Spawn { genome, at, energy } => {
                if world.occupant.get(at).is_none() {
                    // reserve the tile now so a later spawn command this
                    // tick sees it as taken, even though the Blob itself
                    // isn't pushed until after the command loop
                    let id = world.next_id();
                    world.occupant.set(at, Some(id));
                    spawns.push((id, genome, at, energy));
                }
                // else: parent already paid the energy cost in a separate
                // SpendEnergy command — losing a contested tile is the
                // penalty for crowding, not a bug to route around
            }
        }
    }

    for (id, genome, at, energy) in spawns {
        blobs.push(Blob {
            id,
            pos: at,
            dir: Dir::N,
            energy,
            def: 0,
            genome,
            grn: crate::blob::grn::GRN::default(),
        });
    }

    // cull the dead in one pass, after every other effect this tick has settled
    blobs.retain(|b| {
        let alive = !b.is_dead();
        if !alive {
            world.occupant.set(b.pos, None);
            world.food.set(b.pos, true); // corpse becomes food
        }
        alive
    });
}

fn move_blob(blob: &mut Blob, to: Pos, world: &mut World) {
    let old = blob.pos;
    world.occupant.set(old, None);
    world.occupant.set(to, Some(blob.id));
    blob.pos = to;
}

fn eat(blob: &mut Blob, at: Pos, world: &mut World) {
    world.food.set(at, false);
    blob.energy += crate::balance::FOOD_ENERGY_GAIN;
}
