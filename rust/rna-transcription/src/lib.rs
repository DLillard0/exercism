#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.chars().position(|c| {
            match c {
                'A' | 'C' | 'G' | 'T' => false,
                _ => true
            }
        }) {
            Some(i) => Err(i),
            None => Ok(Dna(dna.to_string()))
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(
            self.0.chars().map(|c| {
                match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => panic!()
                }
            }).collect()
        )
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.chars().position(|c| {
            match c {
                'C' | 'G' | 'A' | 'U' => false,
                _ => true
            }
        }) {
            Some(i) => Err(i),
            None => Ok(Rna(rna.to_string()))
        }
    }
}
