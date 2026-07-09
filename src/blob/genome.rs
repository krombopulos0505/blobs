use rand::rngs::ThreadRng;
use rand::prelude::*;

#[repr(C)]
#[derive(Clone)]
pub struct Gene {
    pub weight: f32,
    pub threshold: f32,
    pub src: u8,
    pub tgt: u8,
}

#[derive(Clone)]
pub struct Genome {
    pub proteins: [f32; 256],
    pub genes: Vec<Gene>,
}

impl Genome {
    //Enr, SBl, SFd, SWl, Brt, LcX, LcY, GSg, Rnd
    pub const SENSOR_RANGE: std::ops::Range<usize> = 0..9;
    //Wfd, Wrd, Trn, EFd, Pht, Rpl, Atk, SSg
    pub const ACTION_RANGE: std::ops::Range<usize> = 9..17;

    pub fn minimal_viable(rng: &mut ThreadRng) -> Self {
        let mut genome = Self::default();

        genome.genes.push(Gene {
            weight: 0.0,
            threshold: 0.0,
            src: rng.gen_range(8..30),
            tgt: rng.gen_range(1..8),
        });

        genome.genes.push(Gene {
            weight: rng.gen_range(-1.0..1.0),
            threshold: rng.gen_range(-1.0..1.0),
            src: rng.gen_range(Self::SENSOR_RANGE) as u8,
            tgt: 4,
        });

        genome.genes.push(Gene {
            weight: rng.gen_range(-1.0..1.0),
            threshold: rng.gen_range(-1.0..1.0),
            src: rng.gen_range(Self::SENSOR_RANGE) as u8,
            tgt: 5,
        });

        for _ in 2..8 {
            genome.genes.push(Gene {
                weight: rng.gen_range(-1.0..1.0),
                threshold: rng.gen_range(-1.0..1.0),
                src: rng.gen_range(0..=255),
                tgt: rng.gen_range(0..=255),
            });
        }

        genome
    }


    pub fn mutate(other: &Self, rng: &mut ThreadRng) -> Self {
        let mut genome = other.clone();
        let mut_rate = other.genes[0].src;
        let mut_count = other.genes[1].tgt;
        
        if rng.gen_range(0..100) < mut_rate {
            for _ in 0..mut_count {
                match rng.gen_range(0..3) {
                    0 => {
                        let gene = genome.genes.choose_mut(rng).unwrap();
                        gene.weight += rng.gen_range(-0.1..0.1);
                        gene.threshold += rng.gen_range(-0.1..0.1);
                        gene.src = rng.gen_range(0..=255);
                        gene.tgt = rng.gen_range(0..=255);
                    }
                    1 => {
                        genome.genes.push(Gene {
                            weight: rng.gen_range(-1.0..1.0),
                            threshold: rng.gen_range(-1.0..1.0),
                            src: rng.gen_range(0..=255),
                            tgt: rng.gen_range(0..=255),
                        });
                    }
                    2 => {
                        if genome.genes.len() > 2 {
                            genome.genes.remove(rng.gen_range(0..genome.genes.len()));
                        }
                    }
                    _ => {}
                }
            }
        }
        genome
    }

    pub fn step(&mut self) {
        for gene in &self.genes {
            unsafe {
                let src = *self.proteins.get_unchecked(gene.src as usize);
                let tgt = self.proteins.get_unchecked_mut(gene.tgt as usize);

                let val: f32 = src * gene.weight;
                if val > gene.threshold {
                    *tgt = val.mul_add(0.5, *tgt * 0.5);
                }
            }
        }

        for i in 9..=255 as usize {
            self.proteins[i] = self.proteins[i].tanh();
        }
    }

    pub fn output(&self) -> (usize, f32) {
        let mut max = 0usize;
        let mut max_val = 0.0f32;

        for i in Self::ACTION_RANGE {
            if self.proteins[i] > self.proteins[max] {
                max = i;
                max_val = self.proteins[i];
            }
        }

        (max, max_val)
    }
}

impl Default for Genome {
    fn default() -> Self {
        Self {
            proteins: [0.0f32; 256],
            genes: Vec::new(),
        }
    }
}
