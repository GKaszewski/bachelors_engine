use crate::evolution::genes::*;

#[test]
fn test_red_gene() {
    let mut gene_builder = GeneBuilder::new();
    let red_gene = gene_builder.red().build_gene();

    //AATTC
    let expected_sequence = vec![
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::C,
    ];

    assert_eq!(red_gene.sequence, expected_sequence);
}

#[test]
fn test_green_gene() {
    let mut gene_builder = GeneBuilder::new();
    let green_gene = gene_builder.green().build_gene();

    //ATTTC
    let expected_sequence = vec![
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::C,
    ];

    assert_eq!(green_gene.sequence, expected_sequence);
}

#[test]
fn test_blue_gene() {
    let mut gene_builder = GeneBuilder::new();
    let blue_gene = gene_builder.blue().build_gene();

    //ATCTC
    let expected_sequence = vec![
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::C,
    ];

    assert_eq!(blue_gene.sequence, expected_sequence);
}

#[test]
fn test_yellow_gene() {
    let mut gene_builder = GeneBuilder::new();
    let yellow_gene = gene_builder.yellow().build_gene();

    //ATGTC
    let expected_sequence = vec![
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::C,
    ];

    assert_eq!(yellow_gene.sequence, expected_sequence);
}

#[test]
fn test_purple_gene() {
    let mut gene_builder = GeneBuilder::new();
    let purple_gene = gene_builder.purple().build_gene();

    //ATATC
    let expected_sequence = vec![
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::C,
    ];

    assert_eq!(purple_gene.sequence, expected_sequence);
}

#[test]
fn test_small_size_gene() {
    let mut gene_builder = GeneBuilder::new();
    let small_size_gene = gene_builder.small().build_gene();

    //CCCGT
    let expected_sequence = vec![
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::T,
    ];

    assert_eq!(small_size_gene.sequence, expected_sequence);
}

#[test]
fn test_medium_size_gene() {
    let mut gene_builder = GeneBuilder::new();
    let medium_size_gene = gene_builder.medium().build_gene();

    //CCTGT
    let expected_sequence = vec![
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::T,
    ];

    assert_eq!(medium_size_gene.sequence, expected_sequence);
}

#[test]
fn test_large_size_gene() {
    let mut gene_builder = GeneBuilder::new();
    let large_size_gene = gene_builder.large().build_gene();

    //CCGGT
    let expected_sequence = vec![
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::T,
    ];

    assert_eq!(large_size_gene.sequence, expected_sequence);
}

#[test]
fn test_round_shape_gene() {
    let mut gene_builder = GeneBuilder::new();
    let round_shape_gene = gene_builder.round().build_gene();

    //GGGGA
    let expected_sequence = vec![
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::A,
    ];

    assert_eq!(round_shape_gene.sequence, expected_sequence);
}

#[test]
fn test_square_shape_gene() {
    let mut gene_builder = GeneBuilder::new();
    let square_shape_gene = gene_builder.square().build_gene();

    //GGCGA
    let expected_sequence = vec![
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::A,
    ];

    assert_eq!(square_shape_gene.sequence, expected_sequence);
}

#[test]
fn test_high_temperature_resistance_gene() {
    let mut gene_builder = GeneBuilder::new();
    let high_temperature_resistance_gene = gene_builder.high_temperature_resistance().build_gene();

    //TTTTT
    let expected_sequence = vec![
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
    ];

    assert_eq!(high_temperature_resistance_gene.sequence, expected_sequence);
}

#[test]
fn test_medium_temperature_resistance_gene() {
    let mut gene_builder = GeneBuilder::new();
    let medium_temperature_resistance_gene =
        gene_builder.medium_temperature_resistance().build_gene();

    //TTGTT
    let expected_sequence = vec![
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::T,
    ];

    assert_eq!(
        medium_temperature_resistance_gene.sequence,
        expected_sequence
    );
}

#[test]
fn test_low_temperature_resistance_gene() {
    let mut gene_builder = GeneBuilder::new();
    let low_temperature_resistance_gene = gene_builder.low_temperature_resistance().build_gene();

    //TTATT
    let expected_sequence = vec![
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
    ];

    assert_eq!(low_temperature_resistance_gene.sequence, expected_sequence);
}

#[test]
fn test_no_temperature_resistance_gene() {
    let mut gene_builder = GeneBuilder::new();
    let no_temperature_resistance_gene = gene_builder.no_temperature_resistance().build_gene();

    //TTAAT
    let expected_sequence = vec![
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::T,
    ];

    assert_eq!(no_temperature_resistance_gene.sequence, expected_sequence);
}

#[test]
fn test_high_water_resistance_gene() {
    let mut gene_builder = GeneBuilder::new();
    let high_water_resistance_gene = gene_builder.high_water_resistance().build_gene();

    //AAAAA
    let expected_sequence = vec![
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
    ];

    assert_eq!(high_water_resistance_gene.sequence, expected_sequence);
}

#[test]
fn test_no_water_resistance_gene() {
    let mut gene_builder = GeneBuilder::new();
    let no_water_resistance_gene = gene_builder.no_water_resistance().build_gene();

    //AAATA
    let expected_sequence = vec![
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::A,
    ];

    assert_eq!(no_water_resistance_gene.sequence, expected_sequence);
}

#[test]
fn test_high_metabolism_gene() {
    let mut gene_builder = GeneBuilder::new();
    let high_metabolism_gene = gene_builder.high_metabolism().build_gene();

    //GCGCG
    let expected_sequence = vec![
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::G,
    ];

    assert_eq!(high_metabolism_gene.sequence, expected_sequence);
}

#[test]
fn test_medium_metabolism_gene() {
    let mut gene_builder = GeneBuilder::new();
    let medium_metabolism_gene = gene_builder.medium_metabolism().build_gene();

    //GCGTG
    let expected_sequence = vec![
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::G,
    ];

    assert_eq!(medium_metabolism_gene.sequence, expected_sequence);
}

#[test]
fn test_low_metabolism_gene() {
    let mut gene_builder = GeneBuilder::new();
    let low_metabolism_gene = gene_builder.low_metabolism().build_gene();

    //GCGAG
    let expected_sequence = vec![
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::G,
    ];

    assert_eq!(low_metabolism_gene.sequence, expected_sequence);
}

#[test]
fn test_high_speed_gene() {
    let mut gene_builder = GeneBuilder::new();
    let high_speed_gene = gene_builder.high_speed().build_gene();

    //TTTTT
    let expected_sequence = vec![
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
    ];

    assert_eq!(high_speed_gene.sequence, expected_sequence);
}

#[test]
fn test_medium_speed_gene() {
    let mut gene_builder = GeneBuilder::new();
    let medium_speed_gene = gene_builder.medium_speed().build_gene();

    //TTGTT
    let expected_sequence = vec![
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::T,
    ];

    assert_eq!(medium_speed_gene.sequence, expected_sequence);
}

#[test]
fn test_low_speed_gene() {
    let mut gene_builder = GeneBuilder::new();
    let low_speed_gene = gene_builder.low_speed().build_gene();

    //TTATT
    let expected_sequence = vec![
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
    ];

    assert_eq!(low_speed_gene.sequence, expected_sequence);
}

#[test]
fn test_no_speed_gene() {
    let mut gene_builder = GeneBuilder::new();
    let no_speed_gene = gene_builder.no_speed().build_gene();

    //TTAAT
    let expected_sequence = vec![
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::T,
    ];

    assert_eq!(no_speed_gene.sequence, expected_sequence);
}

#[test]
fn test_high_breeding_rate_gene() {
    let mut gene_builder = GeneBuilder::new();
    let high_breeding_rate_gene = gene_builder.high_breeding_rate().build_gene();

    //CCCCC
    let expected_sequence = vec![
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
    ];

    assert_eq!(high_breeding_rate_gene.sequence, expected_sequence);
}

#[test]
fn test_medium_breeding_rate_gene() {
    let mut gene_builder = GeneBuilder::new();
    let medium_breeding_rate_gene = gene_builder.medium_breeding_rate().build_gene();

    //CCCTC
    let expected_sequence = vec![
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::C,
    ];

    assert_eq!(medium_breeding_rate_gene.sequence, expected_sequence);
}

#[test]
fn test_low_breeding_rate_gene() {
    let mut gene_builder = GeneBuilder::new();
    let low_breeding_rate_gene = gene_builder.low_breeding_rate().build_gene();

    //CCCAC
    let expected_sequence = vec![
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::C,
    ];

    assert_eq!(low_breeding_rate_gene.sequence, expected_sequence);
}

#[test]
fn test_no_breeding_rate_gene() {
    let mut gene_builder = GeneBuilder::new();
    let no_breeding_rate_gene = gene_builder.no_breeding_rate().build_gene();

    //CCCAA
    let expected_sequence = vec![
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::A,
    ];

    assert_eq!(no_breeding_rate_gene.sequence, expected_sequence);
}

#[test]
fn test_long_lifespan_gene() {
    let mut gene_builder = GeneBuilder::new();
    let long_lifespan_gene = gene_builder.long_lifespan().build_gene();

    //GGGGG
    let expected_sequence = vec![
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
    ];

    assert_eq!(long_lifespan_gene.sequence, expected_sequence);
}

#[test]
fn test_medium_lifespan_gene() {
    let mut gene_builder = GeneBuilder::new();
    let medium_lifespan_gene = gene_builder.medium_lifespan().build_gene();

    //GGCGG
    let expected_sequence = vec![
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::G,
    ];

    assert_eq!(medium_lifespan_gene.sequence, expected_sequence);
}

#[test]
fn test_short_lifespan_gene() {
    let mut gene_builder = GeneBuilder::new();
    let short_lifespan_gene = gene_builder.short_lifespan().build_gene();

    //GGAGG
    let expected_sequence = vec![
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::G,
    ];

    assert_eq!(short_lifespan_gene.sequence, expected_sequence);
}

#[test]
fn test_combine_genes_into_one_sequence() {
    let mut gene_builder = GeneBuilder::new();
    let red_gene = gene_builder.red().build_gene();
    let green_gene = gene_builder.green().build_gene();

    let genes = vec![red_gene, green_gene];
    let combined_sequence = combine_genes_into_one_sequence(&genes);

    //AATTCATTTC
    let expected_sequence = vec![
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::C,
    ];

    assert_eq!(combined_sequence, expected_sequence);
}

#[test]
fn test_create_sequence_from_str() {
    let sequence_str = "AATTCATTTC";
    let sequence = create_sequence_from_str(sequence_str);

    //AATTCATTTC
    let expected_sequence = vec![
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::C,
    ];

    assert_eq!(sequence, expected_sequence);
}
