use rand::seq::SliceRandom;
use rand::Rng;

use super::sensor::SENSOR_COUNT;

/// A single regulatory connection, byte-packed so mutation is just
/// "twiddle a byte" instead of separate float/int mutation rules.
/// bytes[0] = src protein index, bytes[1] = tgt protein index,
/// bytes[2] = weight (interpreted as i8, scaled), bytes[3] = threshold (interpreted as i8, scaled).
#[derive(Copy, Clone, Default, Debug)]
pub struct Gene {
    pub bytes: [u8; 4],
}

impl Gene {
    pub fn src(&self) -> usize {
        self.bytes[0] as usize
    }

    pub fn tgt(&self) -> usize {
        self.bytes[1] as usize
    }

    pub fn weight(&self) -> f32 {
        self.bytes[2] as i8 as f32 / 32.0
    }

    pub fn threshold(&self) -> f32 {
        self.bytes[3] as i8 as f32 / 127.0
    }

    fn random(rng: &mut impl Rng) -> Self {
        Self { bytes: [rng.gen(), rng.gen(), rng.gen(), rng.gen()] }
    }
}

/// The inherited blueprint. Mutation rate/count are explicit fields, not
/// smuggled into gene 0 the way the previous draft did -- that made the
/// control knobs both the thing driving mutation and a target mutation
/// could scramble or delete.
#[derive(Clone, Debug)]
pub struct Genome {
    /// probability (out of 255) that a mutation event fires on reproduction
    pub mut_rate: u8,
    /// how many point-mutations to apply when a mutation event fires
    pub mut_count: u8,
    pub genes: Vec<Gene>,
}

impl Genome {
    pub fn minimal_viable(rng: &mut impl Rng) -> Self {
        // named indices into the shared protein array: sensors first, then actions
        let sensor = |i: usize| i as u8;
        let action = |i: usize| (SENSOR_COUNT + i) as u8;

        let genes = vec![
            Gene { bytes: [sensor(2), action(3), 90, 10] }, // SeeFood     -> Eat
            Gene { bytes: [sensor(4), action(4), 90, 10] }, // Brightness  -> Photosynthesize
            Gene { bytes: [sensor(0), action(1), (-40i8) as u8, 10] }, // low Energy  -> WalkRandom
            Gene { bytes: [sensor(0), action(5), 90, 60] }, // high Energy -> Replicate
            Gene::random(rng),
            Gene::random(rng),
        ];

        Self {
            mut_rate: rng.gen_range(8..40),
            mut_count: rng.gen_range(1..4),
            genes,
        }
    }

    pub fn mutate(parent: &Self, rng: &mut impl Rng) -> Self {
        let mut genome = parent.clone();

        if rng.gen_range(0..255u8) < genome.mut_rate {
            for _ in 0..genome.mut_count {
                match rng.gen_range(0..4) {
                    0 => {
                        if let Some(gene) = genome.genes.choose_mut(rng) {
                            let byte_idx = rng.gen_range(0..4);
                            gene.bytes[byte_idx] = rng.gen();
                        }
                    }
                    1 => genome.mut_rate = rng.gen(),
                    2 => genome.genes.push(Gene::random(rng)),
                    3 => {
                        if genome.genes.len() > 2 {
                            let i = rng.gen_range(0..genome.genes.len());
                            genome.genes.remove(i);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        genome
    }
}
