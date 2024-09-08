use genes::{
    combine_genes_into_one_sequence, create_gene_map, get_gene_traits_from_genome_sequence,
    get_only_existing_traits, GeneBuilder, Genome,
};

mod genes;
mod tests;

pub struct Evolution {}

struct Organism {
    genome: Genome,

    food: u32,
    age: u32,
    health: u32,
    lifespan: u32,
}

impl std::fmt::Debug for Organism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let genome_sequence = combine_genes_into_one_sequence(&self.genome.genes);
        let genome_sequence_str = genome_sequence
            .iter()
            .map(|n| char::from(*n))
            .collect::<String>();
        let traits = get_gene_traits_from_genome_sequence(&genome_sequence_str, &create_gene_map());
        let traits = get_only_existing_traits(traits);

        f.debug_struct("Organism")
            .field("genome", &self.genome)
            .field("Genes", &self.genome.genes.len())
            .field("food", &self.food)
            .field("age", &self.age)
            .field("health", &self.health)
            .field("lifespan", &self.lifespan)
            .field("traits", &traits)
            .field("Traits number", &traits.len())
            .finish()
    }
}

pub fn run() {
    let mut gene_builder = GeneBuilder::new();

    let water_gene = gene_builder.high_water_resistance().build_gene();
    let temperature_gene = gene_builder.high_temperature_resistance().build_gene();
    let metabolism_gene = gene_builder.high_metabolism().build_gene();
    let speed_gene = gene_builder.high_speed().build_gene();

    let red_gene = gene_builder.red().build_gene();
    let green_gene = gene_builder.green().build_gene();
    let blue_gene = gene_builder.blue().build_gene();

    let genome = gene_builder.build_genome(vec![
        red_gene,
        green_gene,
        blue_gene,
        water_gene,
        temperature_gene,
        metabolism_gene,
        speed_gene,
    ]);

    let organism = gene_builder.build_organism(genome);

    println!("{:?}", organism);
}
