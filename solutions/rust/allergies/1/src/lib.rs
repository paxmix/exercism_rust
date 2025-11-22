pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

const ALLERGENS: &[(Allergen, u32)] = &[
    (Allergen::Eggs, 1),
    (Allergen::Peanuts, 2),
    (Allergen::Shellfish, 4),
    (Allergen::Strawberries, 8),
    (Allergen::Tomatoes, 16),
    (Allergen::Chocolate, 32),
    (Allergen::Pollen, 64),
    (Allergen::Cats, 128),
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score: score % 256 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        ALLERGENS
            .iter()
            .find(|(a, _)| a == allergen)
            // use bitwise AND to determine if allergen is present is score's binary number
            // Ex: 253 = 11_111_101 in binary, while Peanuts(2) = 10 (00_000_010)
            // bitwise AND return 1 if both binary position is 1
            // therefore allergies(253) is not allergic to Peanuts
            .map(|(_, bit)| self.score & bit != 0)
            .unwrap_or(false)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .iter()
            .filter(|(_, bit)| self.score & bit != 0)
            .map(|(a, _)| *a)
            .collect()
    }
}
