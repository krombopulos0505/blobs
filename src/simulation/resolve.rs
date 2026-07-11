use super::command::Command;
use super::context::SimulationContext;
use crate::balance::*;
use crate::blob::genome::Genome;
use crate::blob::grn::GRN;
use crate::blob::Blob;
use crate::position::{Dir, Pos};
use crate::world::World;

/// Replays this tick's commands against mutable state, then does a single
/// deferred cleanup pass. Spawns and deaths are both buffered rather than
/// applied inline: a new blob appearing (or an old one vanishing)
/// mid-loop would shift indices out from under any later command in the
/// same `cmds` list, since every `Command` addresses blobs by index.
pub fn resolve_phase(ctx: &mut SimulationContext, cmds: Vec<Command>) {
    let mut spawns: Vec<(Genome, Pos, i16)> = Vec::new();

    for cmd in cmds {
        match cmd {
            Command::Move { actor, dir } => move_blob(ctx, actor, dir),
            Command::Turn { actor } => ctx.blobs[actor].dir = ctx.blobs[actor].dir.cw(),
            Command::Eat { actor } => eat(ctx, actor),
            Command::Photosynthesize { actor } => photosynthesize(ctx, actor),
            Command::SpendEnergy { actor, amount } => ctx.blobs[actor].energy -= amount,
            Command::Replicate { actor } => try_replicate(ctx, actor, &mut spawns),
            Command::Attack { actor, target } => attack(ctx, actor, target),
        }
    }

    spawn_children(ctx, spawns);
    cleanup(ctx);
}

fn move_blob(ctx: &mut SimulationContext, actor: usize, dir: Dir) {
    let from = ctx.blobs[actor].pos;
    let to = from.neighbor(dir);

    let is_free = ctx.world.in_bounds(to)
        && ctx.world.occupant.get(to).is_none()
        && !*ctx.world.wall.get(to);
    if !is_free {
        return;
    }

    ctx.world.occupant.set(from, None);
    ctx.world.occupant.set(to, Some(ctx.blobs[actor].id));
    ctx.blobs[actor].pos = to;
}

fn eat(ctx: &mut SimulationContext, actor: usize) {
    let pos = ctx.blobs[actor].pos;
    if *ctx.world.food.get(pos) {
        ctx.world.food.set(pos, false);
        ctx.blobs[actor].energy += FOOD_ENERGY_GAIN;
    }
}

fn photosynthesize(ctx: &mut SimulationContext, actor: usize) {
    let gain = (PHOT_ENERGY_GAIN * ctx.world.brightness()) as i16;
    ctx.blobs[actor].energy += gain;
}

fn attack(ctx: &mut SimulationContext, actor: usize, target: usize) {
    // both indices were resolved during apply_phase against the same
    // (still-unshrunk) blobs vec, so target < blobs.len() always holds;
    // the actor check just skips no-ops if it died earlier this tick
    if ctx.blobs[actor].is_dead() {
        return;
    }
    let damage = (ATTACK_DAMAGE - ctx.blobs[target].def).max(1);
    ctx.blobs[target].energy -= damage;
}

fn try_replicate(ctx: &mut SimulationContext, actor: usize, spawns: &mut Vec<(Genome, Pos, i16)>) {
    let parent = &ctx.blobs[actor];
    if parent.energy < REPLICATION_COST {
        return;
    }
    let Some(spot) = find_empty_neighbor(ctx.world, parent.pos) else {
        return;
    };

    let genome = Genome::mutate(&parent.genome, ctx.rng);
    spawns.push((genome, spot, REPLICATION_COST));
    ctx.blobs[actor].energy -= REPLICATION_COST;
}

fn find_empty_neighbor(world: &World, from: Pos) -> Option<Pos> {
    (0..8)
        .map(|i| from.neighbor(Dir::from_index(i)))
        .find(|p| world.in_bounds(*p) && world.occupant.get(*p).is_none() && !*world.wall.get(*p))
}

fn spawn_children(ctx: &mut SimulationContext, spawns: Vec<(Genome, Pos, i16)>) {
    for (genome, pos, energy) in spawns {
        // the neighbor slot chosen in try_replicate may have filled up
        // (another spawn, another move) by the time we get here
        if ctx.world.occupant.get(pos).is_some() {
            continue;
        }

        let id = ctx.world.next_id();
        ctx.world.occupant.set(pos, Some(id));
        ctx.blobs.push(Blob {
            id,
            pos,
            dir: Dir::N,
            energy,
            def: 0,
            genome,
            grn: GRN::default(),
        });
    }
}

fn cleanup(ctx: &mut SimulationContext) {
    for blob in ctx.blobs.iter() {
        if blob.is_dead() {
            ctx.world.occupant.set(blob.pos, None);
            ctx.world.food.set(blob.pos, true); // corpse becomes food
        }
    }
    ctx.blobs.retain(|b| !b.is_dead());
}
