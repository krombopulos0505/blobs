use rand::Rng;

use super::decide::Decision;
use super::resolve::Command;
use crate::balance::*;
use crate::blob::Blob;
use crate::blob::action::Action;
use crate::blob::genome::Genome;
use crate::position::{Dir, Pos};
use crate::world::World;

pub fn apply_phase(
    blobs: &[Blob], world: &World,
    decisions: &[Decision], rng: &mut impl Rng,
) -> &mut Vec<Command> {
    let mut cmds = Vec::new();

    for d in decisions {
        let blob = &blobs[d.blob_idx];
        cmds.push(Command::SpendEnergy {
            blob_idx: d.blob_idx,
            amount: METABOLISM,
        });

        match d.kind {
            Action::Turn => turn(blob_idx, &mut cmds);
            Action::WalkForward | Action::WalkRandom => {
                let dir = if matches!(d.kind, WalkRandom) {
                    Dir::random(rng)
                } else {
                    blob.dir
                }
                walk(blob_idx, dir, &mut cmds);
            }
            Action::Eat => eat(blob_idx, &mut cmds);
            Action::Photosynthesize => photosynthesize(blob_idx, &mut cmds);
            Action::Replicate => replicate(blob_idx, &mut cmds);
            Action::Attack => attack(blob_idx, &mut cmds);
            Action::SetSignal => {blob_idx, &mut cmds},
        }
    }
    cmds
}

fn find_empty_neighbor(world: &World, from: Pos) -> Option<Pos> {
    (0..8)
        .map(|i| from.neighbor(Dir::from_index(i)))
        .find(|p| world.in_bounds(*p) && world.occupant.get(*p).is_none() && !*world.wall.get(*p))
}

fn turn(blob_idx: usize, cmds: Vec<Command>) {
    cmds.push(Command::Turn { blob_idx });
}

fn walk(blob_idx: usize, dir: Dir, cmds: &mut Vec<Command>) {
    cmds.push(Command::Move { blob_idx, dir });
    cmds.push(Command::SpendEnergy {
        blob_idx
        amount: ACTION_COST,
    });
}

fn eat(blob_idx: usize, cmds: &mut Vec<Command>) {
    cmds.push(Command::Eat { blob_idx });
    cmds.push(Command::SpendEnergy {
        blob_idx
        amount: ACTION_COST,
    });
}

fn photosynthesize(blob_idx: usize, cmds: &mut Vec<Command>) {
    cmds.push(Command::Photosynthesize { blob_idx });
    cmds.push(Command::SpendEnergy {
        blob_idx
        amount: ACTION_COST,
    });
}

fn replicate(blob_idx: usize, cmds: &mut Vec<Command>) {
    cmds.push(Command::Replicate { blob_idx });
}

fn attack(blob_idx: usize, cmds: &mut Vec<Command>) {
    cmds.push(Command::Damage { blob_idx });
    cmds.push(Command::SpendEnergy {
        blob_idx,
        amount: ATTACK_COST,
    });
}
