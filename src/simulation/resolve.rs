use crate::blob::genome::Genome;
use crate::blob::{Blob, BlobId};
use crate::position::{Dir, Pos};
use crate::world::World;

pub enum Command {
    Walk(usize, Dir),
    Turn(usize), Eat(usize),
    Photosynthesize(usize),
    Replicate(usize),
    Attack(usize), SpendEnergy(usize, i16),
}

pub fn resolve_phase(blobs: &mut Vec<Blob>, world: &mut World, cmds: Vec<Command>) {
    let mut spawns = Vec::new();

    for cmd in cmds {
        match cmd {
            Command::Walk(idx, dir) => walk(blobs[idx], dir, world),
            Command::Turn(idx) => turn(blobs[idx]),
            Command::Eat(idx) => eat(blobs[idx], world),
            Command::Photosynthesize(idx) => photosynthesize(blobs[idx], world),
            Command::SpendEnergy(idx, amount) => blobs[idx].energy -= amount,
            Command::Replicate(idx) => replicate(blobs[idx], blobs, world),
            Command::Attack(idx) => attack(blobs[idx], blobs, world);
        }
    }

    spawn(blobs, spawns);

    cleanup(blobs);
}

fn walk(blob: &mut Blob, dir: Dir, world: &mut World) {
    let npos = blob.pos.neighbor(dir);
    let is_empty = world.in_bounds(npos) &&
        world.occupant.get(npos).is_none() &&
        !world.wall.get(npos);
    if is_empty {
        blob.pos = npos;
    }
}

//create all other action functions pls

fn cleanup(blobs: &mut Vec<Blob>) {
    blobs.retain(|b| {
        let alive = !b.is_dead();
        if !alive {
            world.occupant.set(b.pos, None);
            world.food.set(b.pos, true); // corpse becomes food
        }
        alive
    });
}

fn spawn(blobs: &mut Vec<Blob>, spawns: Vec<(BlobId, Genome, Pos, i16)>) {
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
}
