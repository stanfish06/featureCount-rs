use std::fmt;

use bam::RecordWriter;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

pub enum dna_base {
    A,
    T,
    C,
    G,
}

impl Distribution<dna_base> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> dna_base {
        let index: u8 = rng.random_range(0..4);
        match index {
            0 => dna_base::A,
            1 => dna_base::T,
            2 => dna_base::C,
            3 => dna_base::G,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for dna_base {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_char = match self {
            dna_base::A => 'A',
            dna_base::T => 'T',
            dna_base::C => 'C',
            dna_base::G => 'G',
        };
        write!(f, "{}", base_char)
    }
}

pub fn generate_rand_sam(n: u16) {
    let mut rng = rand::rng();
    let rand_base: dna_base = StandardUniform.sample(&mut rng);
    println!("{}", rand_base);
}
