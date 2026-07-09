#[repr(C)]
pub struct Gene {
    pub weight: f32,
    pub threshold: f32,
    pub src: u8,
    pub tgt: u8,
}

pub struct Genome {
    pub proteins: [f32; 256],
    pub genes: Vec<Gene>,
}

impl Genome {
    //Enr, SBl, SFd, SWl, Brt, LcX, LcY, GSg, Rnd
    pub const SENSOR_RANGE: iter = 0..9;
    //Wfd, Wrd, Trn, EFd, Pht, Rpl, Atk, SSg
    pub const ACTION_RANGE: iter = 9..17;

    pub fn step(&mut self) {
        /*
        self.genes.iter()
            .for_each(|gene| {
                let val = self.proteins[gene.src] * gene.weight;
                if val > gene.threshold {
                    self.proteins[gene.tgt] = 0.5*val + 0.5*self.proteins[gene.tgt];
                }
            });
        */

        for gene in &self.genes {
            unsafe {
                let src = *self.proteins.get_unchecked(gene.src);
                let tgt = self.proteins.get_unchecked(gene.tgt);

                let val = src * gene.weight;
                if val > gene.threshold {
                    *tgt = val.mul_add(0.5, *tgt * 0.5);
                }
            }
        }
    }

    pub fn output(&self) -> (usize, f32) {
        let mut max = 0usize;
        let mut max_val = 0.0f32;

        for i in ACTION_RANGE {
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
