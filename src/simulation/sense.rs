use rand::Rng;

use crate::blob::sensor::Sensor;
use crate::blob::Blob;
use crate::position::Pos;
use crate::world::World;

pub fn sense_phase(blobs: &mut [Blob], world: &World, rng: &mut impl Rng) {
    let width = world.width();
    let height = world.height();

    for blob in blobs.iter_mut() {
        let energy = blob.energy;
        let pos = blob.pos;
        let dir = blob.dir;
        let p = blob.grn.proteins_mut();

        p[Sensor::Energy as usize] =
            (energy - crate::balance::BASE_ENERGY) as f32 / crate::balance::BASE_ENERGY as f32;
        p[Sensor::SeeBlob as usize] = sees(world, pos, dir, |t| world.occupant.get(t).is_some());
        p[Sensor::SeeFood as usize] = sees(world, pos, dir, |t| *world.food.get(t));
        p[Sensor::SeeWall as usize] = sees(world, pos, dir, |t| *world.wall.get(t));
        p[Sensor::Brightness as usize] = world.brightness();
        p[Sensor::LocX as usize] = (2 * pos.x - width) as f32 / width as f32;
        p[Sensor::LocY as usize] = (2 * pos.y - height) as f32 / height as f32;
        p[Sensor::GetSignal as usize] = 0.0; // wire up once signaling/scent exists
        p[Sensor::Random as usize] = rng.gen_range(-1.0..1.0);
    }
}

fn sees(world: &World, pos: Pos, dir: crate::position::Dir, check: impl Fn(Pos) -> bool) -> f32 {
    let n = pos.neighbor(dir);
    if world.in_bounds(n) && check(n) {
        1.0
    } else {
        -1.0
    }
}
