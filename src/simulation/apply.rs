use super::command::Command;
use super::context::SimulationContext;
use super::decide::Decision;
use crate::balance::*;
use crate::blob::action::Action;
use crate::position::Dir;

/// Turns this tick's `Decision`s into `Command`s. Runs with read access to
/// `blobs`/`world` (plus a mutable rng) -- any target resolution that
/// needs the current world state (e.g. "who's in front of me") happens
/// here, once, so `resolve_phase` never has to search.
pub fn apply_phase(ctx: &mut SimulationContext, decisions: &[Decision]) -> Vec<Command> {
    let mut cmds = Vec::with_capacity(decisions.len() * 2);

    for d in decisions {
        let actor = d.blob_idx;
        cmds.push(Command::SpendEnergy {
            actor,
            amount: METABOLISM,
        });

        match d.kind {
            Action::Turn => cmds.push(Command::Turn { actor }),

            Action::WalkForward | Action::WalkRandom => {
                let dir = if d.kind == Action::WalkRandom {
                    Dir::random(ctx.rng)
                } else {
                    ctx.blobs[actor].dir
                };
                cmds.push(Command::Move { actor, dir });
                cmds.push(Command::SpendEnergy {
                    actor,
                    amount: ACTION_COST,
                });
            }

            Action::Eat => {
                cmds.push(Command::Eat { actor });
                cmds.push(Command::SpendEnergy {
                    actor,
                    amount: ACTION_COST,
                });
            }

            Action::Photosynthesize => {
                cmds.push(Command::Photosynthesize { actor });
                cmds.push(Command::SpendEnergy {
                    actor,
                    amount: ACTION_COST,
                });
            }

            Action::Replicate => {
                // energy is checked again in resolve_phase (it may have
                // been spent by an earlier command this same tick), this
                // just avoids emitting doomed commands for the common case
                if ctx.blobs[actor].energy >= REPLICATION_COST {
                    cmds.push(Command::Replicate { actor });
                }
            }

            Action::Attack => {
                if let Some(target) = facing_occupant(ctx, actor) {
                    cmds.push(Command::Attack { actor, target });
                    cmds.push(Command::SpendEnergy {
                        actor,
                        amount: ATTACK_COST,
                    });
                }
            }

            Action::SetSignal => {
                // wire up once signaling/scent exists
            }
        }
    }

    cmds
}

/// Index (not id) of whatever blob currently occupies the tile the actor
/// is facing, if any.
fn facing_occupant(ctx: &SimulationContext, actor: usize) -> Option<usize> {
    let blob = &ctx.blobs[actor];
    let target_pos = blob.pos.neighbor(blob.dir);
    if !ctx.world.in_bounds(target_pos) {
        return None;
    }
    let target_id = (*ctx.world.occupant.get(target_pos))?;
    ctx.blobs.iter().position(|b| b.id == target_id)
}
