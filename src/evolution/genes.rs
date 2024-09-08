use std::collections::HashMap;

use super::Organism;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Nucleotide {
    A,
    T,
    C,
    G,
}

impl From<u8> for Nucleotide {
    fn from(item: u8) -> Self {
        match item {
            0 => Nucleotide::A,
            1 => Nucleotide::T,
            2 => Nucleotide::C,
            3 => Nucleotide::G,
            _ => panic!("Invalid nucleotide value"),
        }
    }
}

impl From<Nucleotide> for u8 {
    fn from(item: Nucleotide) -> Self {
        match item {
            Nucleotide::A => 0,
            Nucleotide::T => 1,
            Nucleotide::C => 2,
            Nucleotide::G => 3,
        }
    }
}

impl From<char> for Nucleotide {
    fn from(item: char) -> Self {
        match item {
            'A' => Nucleotide::A,
            'T' => Nucleotide::T,
            'C' => Nucleotide::C,
            'G' => Nucleotide::G,
            _ => panic!("Invalid nucleotide value"),
        }
    }
}

impl From<Nucleotide> for char {
    fn from(item: Nucleotide) -> Self {
        match item {
            Nucleotide::A => 'A',
            Nucleotide::T => 'T',
            Nucleotide::C => 'C',
            Nucleotide::G => 'G',
        }
    }
}

impl From<&str> for Nucleotide {
    fn from(item: &str) -> Self {
        match item {
            "A" => Nucleotide::A,
            "T" => Nucleotide::T,
            "C" => Nucleotide::C,
            "G" => Nucleotide::G,
            _ => panic!("Invalid nucleotide value"),
        }
    }
}

impl From<Nucleotide> for &str {
    fn from(item: Nucleotide) -> Self {
        match item {
            Nucleotide::A => "A",
            Nucleotide::T => "T",
            Nucleotide::C => "C",
            Nucleotide::G => "G",
        }
    }
}

impl From<String> for Nucleotide {
    fn from(item: String) -> Self {
        match item.as_str() {
            "A" => Nucleotide::A,
            "T" => Nucleotide::T,
            "C" => Nucleotide::C,
            "G" => Nucleotide::G,
            _ => panic!("Invalid nucleotide value"),
        }
    }
}

impl From<Nucleotide> for String {
    fn from(item: Nucleotide) -> Self {
        match item {
            Nucleotide::A => "A".to_string(),
            Nucleotide::T => "T".to_string(),
            Nucleotide::C => "C".to_string(),
            Nucleotide::G => "G".to_string(),
        }
    }
}

pub type Sequence = Vec<Nucleotide>;

pub fn create_sequence_from_str(sequence: &str) -> Sequence {
    sequence.chars().map(|c| Nucleotide::from(c)).collect()
}

pub fn combine_genes_into_one_sequence(genes: &Vec<Gene>) -> Sequence {
    genes.iter().map(|g| g.sequence.clone()).flatten().collect()
}

#[derive(Debug, Clone)]
pub struct Gene {
    pub sequence: Sequence,
}

#[derive(Debug, Clone)]
pub struct Genome {
    pub genes: Vec<Gene>,
}

pub const GENE_LENGTH: usize = 5; // length of a gene sequence

/// Gene map and their traits
/// gene segment | trait | gene value | trait value/effect
/// 0-4          | color | AATTC      | red
/// 0-4          | color | ATTTC      | green
/// 0-4          | color | ATCTC      | blue
/// 0-4          | color | ATGTC      | yellow
/// 0-4          | color | ATATC      | purple
///
/// 5-9          | size  | CCCGT      | small
/// 5-9          | size  | CCTGT      | medium
/// 5-9          | size  | CCGGT      | large
///
/// 10-14        | shape | GGGGA      | round
/// 10-14        | shape | GGCGA      | square
///
/// 15-19        | temperature resistance | TTTTT | high (can survive in extreme heat and cold)
/// 15-19        | temperature resistance | TTGTT | medium (can survive in moderate heat and cold)
/// 15-19        | temperature resistance | TTATT | low (can only survive in moderate heat)
/// 15-19        | temperature resistance | TTAAT | none (cannot survive at all)
///
/// 20-24        | water resistance | AAAAA | high (can survive in water)
/// 20-24        | water resistance | AAATA | none (cannot survive in water)
///
/// 25-29        | metabolism | GCGCG | high (can survive on very little food)
/// 25-29        | metabolism | GCGTG | medium (can survive on moderate food)
/// 25-29        | metabolism | GCGAG | low (needs a lot of food)
///
/// 30-34        | speed | TTTTT | high (can move very fast)
/// 30-34        | speed | TTGTT | medium (can move at a moderate pace)
/// 30-34        | speed | TTATT | low (can only move slowly)
/// 30-34        | speed | TTAAT | none (cannot move at all)
///
/// 35-39        | breeding rate | CCCCC | high (can reproduce quickly)
/// 35-39        | breeding rate | CCCTC | medium (can reproduce at a moderate pace)
/// 35-39        | breeding rate | CCCAC | low (reproduces slowly)
/// 35-39        | breeding rate | CCCAA | none (cannot reproduce)
///
/// 40-44        | lifespan | GGGGG | long (can live for a long time)
/// 40-44        | lifespan | GGCGG | medium (can live for a moderate amount of time)
/// 40-44        | lifespan | GGAGG | short (lives for a short amount of time)
#[derive(Debug, Clone)]
pub enum GeneTrait {
    Color(GeneValue),
    Size(GeneValue),
    Shape(GeneValue),
    TemperatureResistance(GeneValue),
    WaterResistance(GeneValue),
    Metabolism(GeneValue),
    Speed(GeneValue),
    BreedingRate(GeneValue),
    Lifespan(GeneValue),
}

#[derive(Debug, Clone)]
pub enum GeneValue {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Small,
    Medium,
    Large,
    Round,
    Square,
    HighTemperatureResistance,
    MediumTemperatureResistance,
    LowTemperatureResistance,
    NoTemperatureResistance,
    HighWaterResistance,
    NoWaterResistance,
    HighMetabolism,
    MediumMetabolism,
    LowMetabolism,
    HighSpeed,
    MediumSpeed,
    LowSpeed,
    NoSpeed,
    HighBreedingRate,
    MediumBreedingRate,
    LowBreedingRate,
    NoBreedingRate,
    LongLifespan,
    MediumLifespan,
    ShortLifespan,
}

pub fn create_gene_map() -> HashMap<String, GeneTrait> {
    let mut gene_map = HashMap::new();

    gene_map.insert("AATTC".to_string(), GeneTrait::Color(GeneValue::Red));
    gene_map.insert("ATTTC".to_string(), GeneTrait::Color(GeneValue::Green));
    gene_map.insert("ATCTC".to_string(), GeneTrait::Color(GeneValue::Blue));
    gene_map.insert("ATGTC".to_string(), GeneTrait::Color(GeneValue::Yellow));
    gene_map.insert("ATATC".to_string(), GeneTrait::Color(GeneValue::Purple));

    gene_map.insert("CCCGT".to_string(), GeneTrait::Size(GeneValue::Small));
    gene_map.insert("CCTGT".to_string(), GeneTrait::Size(GeneValue::Medium));
    gene_map.insert("CCGGT".to_string(), GeneTrait::Size(GeneValue::Large));

    gene_map.insert("GGGGA".to_string(), GeneTrait::Shape(GeneValue::Round));
    gene_map.insert("GGCGA".to_string(), GeneTrait::Shape(GeneValue::Square));

    gene_map.insert(
        "TTTTT".to_string(),
        GeneTrait::TemperatureResistance(GeneValue::HighTemperatureResistance),
    );
    gene_map.insert(
        "TTGTT".to_string(),
        GeneTrait::TemperatureResistance(GeneValue::MediumTemperatureResistance),
    );
    gene_map.insert(
        "TTATT".to_string(),
        GeneTrait::TemperatureResistance(GeneValue::LowTemperatureResistance),
    );
    gene_map.insert(
        "TTAAT".to_string(),
        GeneTrait::TemperatureResistance(GeneValue::NoTemperatureResistance),
    );

    gene_map.insert(
        "AAAAA".to_string(),
        GeneTrait::WaterResistance(GeneValue::HighWaterResistance),
    );
    gene_map.insert(
        "AAATA".to_string(),
        GeneTrait::WaterResistance(GeneValue::NoWaterResistance),
    );

    gene_map.insert(
        "GCGCG".to_string(),
        GeneTrait::Metabolism(GeneValue::HighMetabolism),
    );
    gene_map.insert(
        "GCGTG".to_string(),
        GeneTrait::Metabolism(GeneValue::MediumMetabolism),
    );
    gene_map.insert(
        "GCGAG".to_string(),
        GeneTrait::Metabolism(GeneValue::LowMetabolism),
    );

    gene_map.insert("TTTTT".to_string(), GeneTrait::Speed(GeneValue::HighSpeed));
    gene_map.insert(
        "TTGTT".to_string(),
        GeneTrait::Speed(GeneValue::MediumSpeed),
    );
    gene_map.insert("TTATT".to_string(), GeneTrait::Speed(GeneValue::LowSpeed));
    gene_map.insert("TTAAT".to_string(), GeneTrait::Speed(GeneValue::NoSpeed));

    gene_map.insert(
        "CCCCC".to_string(),
        GeneTrait::BreedingRate(GeneValue::HighBreedingRate),
    );
    gene_map.insert(
        "CCCTC".to_string(),
        GeneTrait::BreedingRate(GeneValue::MediumBreedingRate),
    );
    gene_map.insert(
        "CCCAC".to_string(),
        GeneTrait::BreedingRate(GeneValue::LowBreedingRate),
    );
    gene_map.insert(
        "CCCAA".to_string(),
        GeneTrait::BreedingRate(GeneValue::NoBreedingRate),
    );

    gene_map.insert(
        "GGGGG".to_string(),
        GeneTrait::Lifespan(GeneValue::LongLifespan),
    );
    gene_map.insert(
        "GGCGG".to_string(),
        GeneTrait::Lifespan(GeneValue::MediumLifespan),
    );
    gene_map.insert(
        "GGAGG".to_string(),
        GeneTrait::Lifespan(GeneValue::ShortLifespan),
    );

    gene_map
}

pub fn get_gene_traits_from_genome_sequence(
    genome_sequence: &str,
    gene_map: &HashMap<String, GeneTrait>,
) -> Vec<Option<GeneTrait>> {
    let mut traits = Vec::new();

    for i in (0..genome_sequence.len()).step_by(GENE_LENGTH) {
        let segment = &genome_sequence[i..i + GENE_LENGTH];
        let gene_trait = gene_map.get(segment);

        traits.push(gene_trait.cloned());
    }

    traits
}

pub fn get_only_existing_traits(traits: Vec<Option<GeneTrait>>) -> Vec<GeneTrait> {
    traits.into_iter().filter_map(|t| t).collect()
}

pub fn create_organism_based_on_genome(
    genome: Genome,
    gene_map: &HashMap<String, GeneTrait>,
) -> Organism {
    let genome_sequence = combine_genes_into_one_sequence(&genome.genes);
    let genome_sequence_str = genome_sequence
        .iter()
        .map(|n| char::from(*n))
        .collect::<String>();
    let traits = get_gene_traits_from_genome_sequence(&genome_sequence_str, gene_map);
    let traits = get_only_existing_traits(traits);

    let lifespan = traits
        .iter()
        .filter_map(|t| match t {
            GeneTrait::Lifespan(GeneValue::LongLifespan) => Some(100),
            GeneTrait::Lifespan(GeneValue::MediumLifespan) => Some(50),
            GeneTrait::Lifespan(GeneValue::ShortLifespan) => Some(25),
            _ => None,
        })
        .next()
        .unwrap_or(50);

    let health = traits
        .iter()
        .filter_map(|t| match t {
            GeneTrait::Metabolism(GeneValue::HighMetabolism) => Some(100),
            GeneTrait::Metabolism(GeneValue::MediumMetabolism) => Some(50),
            GeneTrait::Metabolism(GeneValue::LowMetabolism) => Some(25),
            _ => None,
        })
        .next()
        .unwrap_or(50);

    let food = 10;

    Organism {
        genome,
        food,
        age: 0,
        health,
        lifespan,
    }
}

pub struct GeneBuilder {
    gene_map: HashMap<String, GeneTrait>,
    gene: Gene,
}

impl GeneBuilder {
    pub fn new() -> Self {
        GeneBuilder {
            gene_map: create_gene_map(),
            gene: Gene {
                sequence: Vec::new(),
            },
        }
    }

    pub fn build_gene_from_sequence(&self, sequence: &str) -> Gene {
        Gene {
            sequence: create_sequence_from_str(sequence),
        }
    }

    pub fn build_gene(&self) -> Gene {
        self.gene.clone()
    }

    pub fn red(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("AATTC");
        self
    }

    pub fn green(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("ATTTC");
        self
    }

    pub fn blue(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("ATCTC");
        self
    }

    pub fn yellow(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("ATGTC");
        self
    }

    pub fn purple(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("ATATC");
        self
    }

    pub fn small(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("CCCGT");
        self
    }

    pub fn medium(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("CCTGT");
        self
    }

    pub fn large(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("CCGGT");
        self
    }

    pub fn round(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("GGGGA");
        self
    }

    pub fn square(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("GGCGA");
        self
    }

    pub fn high_temperature_resistance(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("TTTTT");
        self
    }

    pub fn medium_temperature_resistance(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("TTGTT");
        self
    }

    pub fn low_temperature_resistance(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("TTATT");
        self
    }

    pub fn no_temperature_resistance(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("TTAAT");
        self
    }

    pub fn high_water_resistance(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("AAAAA");
        self
    }

    pub fn no_water_resistance(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("AAATA");
        self
    }

    pub fn high_metabolism(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("GCGCG");
        self
    }

    pub fn medium_metabolism(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("GCGTG");
        self
    }

    pub fn low_metabolism(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("GCGAG");
        self
    }

    pub fn high_speed(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("TTTTT");
        self
    }

    pub fn medium_speed(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("TTGTT");
        self
    }

    pub fn low_speed(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("TTATT");
        self
    }

    pub fn no_speed(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("TTAAT");
        self
    }

    pub fn high_breeding_rate(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("CCCCC");
        self
    }

    pub fn medium_breeding_rate(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("CCCTC");
        self
    }

    pub fn low_breeding_rate(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("CCCAC");
        self
    }

    pub fn no_breeding_rate(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("CCCAA");
        self
    }

    pub fn long_lifespan(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("GGGGG");
        self
    }

    pub fn medium_lifespan(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("GGCGG");
        self
    }

    pub fn short_lifespan(&mut self) -> &mut Self {
        self.gene.sequence = create_sequence_from_str("GGAGG");
        self
    }

    pub fn build_genes_from_sequences(&self, sequences: Vec<&str>) -> Vec<Gene> {
        sequences
            .iter()
            .map(|s| self.build_gene_from_sequence(s))
            .collect()
    }

    pub fn build_genome(&self, genes: Vec<Gene>) -> Genome {
        Genome { genes }
    }

    pub fn build_organism(&self, genome: Genome) -> Organism {
        create_organism_based_on_genome(genome, &self.gene_map)
    }
}
