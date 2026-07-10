use rand::Rng;

use super::decide::Decision;
use super::resolve::Command;
use crate::balance::*;
use crate::blob::action::Action;
use crate::blob::genome::Genome;
use crate::blob::Blob;
use crate::position::{Dir, Pos};
use crate::world::World;

pub fn apply_phase(
    blobs: &[Blob],
    world: &World,
    decisions: &[Decision],
    rng: &mut impl Rng,
) -> Vec<Command> {
    let mut cmds = Vec::new();

    for d in decisions {
        let blob = &blobs[d.blob_idx];
        cmds.push(Command::SpendEnergy { blob_idx: d.blob_idx, amount: METABOLISM });

        match d.kind {
            Action::Turn => {
                cmds.push(Command::Turn { blob_idx: d.blob_idx, dir: blob.dir.cw() });
            }
            Action::WalkForward | Action::WalkRandom => {
                let dir = if d.kind == Action::WalkRandom { Dir::random(rng) } else { blob.dir };
                let target = blob.pos.neighbor(dir);
                if world.in_bounds(target)
                    && !*world.wall.get(target)
                    && world.occupant.get(target).is_none()
                {
                    cmds.push(Command::Move { blob_idx: d.blob_idx, to: target });
                    cmds.push(Command::SpendEnergy { blob_idx: d.blob_idx, amount: ACTION_COST });
                }
            }
            Action::Eat => {
                if *world.food.get(blob.pos) {
                    cmds.push(Command::Eat { blob_idx: d.blob_idx, at: blob.pos });
                    cmds.push(Command::SpendEnergy { blob_idx: d.blob_idx, amount: ACTION_COST });
                }
            }
            Action::Photosynthesize => {
                let gain = (world.brightness() * PHOT_ENERGY_GAIN) as i16;
                cmds.push(Command::Photosynthesize { blob_idx: d.blob_idx, gain });
                cmds.push(Command::SpendEnergy { blob_idx: d.blob_idx, amount: ACTION_COST });
            }
            Action::Replicate => {
                if let Some(spot) = find_empty_neighbor(world, blob.pos) {
                    let child_energy = blob.energy / 2;
                    cmds.push(Command::Spawn {
                        genome: Genome::mutate(&blob.genome, rng),
                        at: spot,
                        energy: child_energy,
                    });
                    cmds.push(Command::SpendEnergy {
                        blob_idx: d.blob_idx,
                        amount: blob.energy / 2 + REPLICATION_COST,
                    });
                }
            }
            Action::Attack => {
                let target = blob.pos.neighbor(blob.dir);
                if world.in_bounds(target) {
                    if let Some(target_id) = *world.occupant.get(target) {
                        let damage = rng.gen_range(0..BASE_ENERGY / 2) - blob.def;
                        cmds.push(Command::Damage { target_id, amount: damage });
                        cmds.push(Command::SpendEnergy { blob_idx: d.blob_idx, amount: ATTACK_COST });
                    }
                }
            }
            Action::SetSignal => {
                // e.g. cmds.push(Command::EmitScent { at: blob.pos, strength: d.strength });
                let _ = d.strength;
            }
        }
    }
    cmds
}

fn find_empty_neighbor(world: &World, from: Pos) -> Option<Pos> {
    (0..8)
        .map(|i| from.neighbor(Dir::from_index(i)))
        .find(|p| world.in_bounds(*p) && world.occupant.get(*p).is_none() && !*world.wall.get(*p))
}
