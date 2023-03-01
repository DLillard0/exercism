use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match (is_valid_nucleotide(nucleotide), check_dna_valid(dna)) {
        (false, _) => Err(nucleotide),
        (_, Err(c)) => Err(c),
        (_, _) => Ok(dna.chars().filter(|&c| c == nucleotide).count()),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    match check_dna_valid(dna) {
        Err(c) => Err(c),
        _ => {
            let mut map = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
            dna.chars().for_each(|c| {
                map.entry(c).and_modify(|v| *v += 1);
            });
            Ok(map)
        }
    }
}

fn check_dna_valid(dna: &str) -> Result<(), char> {
    match dna.chars().find(|&c| !is_valid_nucleotide(c)) {
        Some(c) => Err(c),
        _ => Ok(()),
    }
}

fn is_valid_nucleotide(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}
