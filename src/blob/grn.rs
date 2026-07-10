use super::action::{Action, ACTION_COUNT};
use super::genome::Genome;
use super::sensor::SENSOR_COUNT;

/// Runtime state of a blob's gene regulatory network. `Genome` is the
/// inherited blueprint (immutable except at reproduction); `GRN` is the
/// live protein soup that blueprint drives, reset fresh per blob.
#[derive(Clone)]
pub struct GRN {
    pub proteins: [f32; 256],
}

impl GRN {
    pub fn proteins_mut(&mut self) -> &mut [f32; 256] {
        &mut self.proteins
    }

    pub fn step(&mut self, genome: &Genome) {
        let mut delta = [0.0f32; 256];

        for gene in &genome.genes {
            let src = gene.src();
            let tgt = gene.tgt();
            if self.proteins[src] > gene.threshold() {
                delta[tgt] += gene.weight() * self.proteins[src];
            }
        }

        // sensors (0..SENSOR_COUNT) are overwritten fresh by sense_phase each
        // tick, so only the regulatory/action range decays and integrates here
        for i in SENSOR_COUNT..256 {
            self.proteins[i] = (0.5 * self.proteins[i] + 0.5 * delta[i].clamp(-1.0, 1.0))
                .clamp(-1.0, 1.0);
        }
    }

    pub fn output(&self) -> (Action, f32) {
        let mut best_idx = 0;
        let mut best_val = self.proteins[SENSOR_COUNT];

        for i in 1..ACTION_COUNT {
            let val = self.proteins[SENSOR_COUNT + i];
            if val.abs() > best_val.abs() {
                best_idx = i;
                best_val = val;
            }
        }

        (Action::ALL[best_idx], best_val)
    }
}

impl Default for GRN {
    fn default() -> Self {
        Self { proteins: [0.0f32; 256] }
    }
}
