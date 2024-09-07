use crate::engine::*;
use rand::Rng;

pub struct Evolution {}

enum Nucleotide {
    A,
    T,
    C,
    G,
}

type Sequence = Vec<Nucleotide>;

struct Genome {
    sequences: Vec<Sequence>,
}

struct Organism {
    genome: Genome,
}

fn create_random_genome() -> Genome {
    let mut rng = rand::thread_rng();

    let sequences = (0..10)
        .map(|_| {
            (0..10)
                .map(|_| match rng.gen_range(0..4) {
                    0 => Nucleotide::A,
                    1 => Nucleotide::T,
                    2 => Nucleotide::C,
                    3 => Nucleotide::G,
                    _ => panic!("Invalid nucleotide"),
                })
                .collect()
        })
        .collect();

    Genome { sequences }
}

fn create_population(size: u32) -> Vec<Entity> {
    let mut entities = Vec::new();

    entities
}
