use self::Allergen::*;

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

const ALLERGEN_LIST: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.score % 256) & *allergen as u32 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGEN_LIST.iter().filter(|v| self.is_allergic_to(v)).cloned().collect()
    }
}
