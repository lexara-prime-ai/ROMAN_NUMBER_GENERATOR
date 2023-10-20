#[derive(Debug)]
pub enum RomanNumbers {
    I = 1,
    IV = 4,
    V = 5,
    IX = 9,
    X = 10,
}

impl RomanNumbers {
    pub fn match_values(&self) -> Self {
        match self {
            Self::I => RomanNumbers::I,
            Self::IV => RomanNumbers::IV,
            Self::V => RomanNumbers::V,
            Self::IX => RomanNumbers::IX,
            Self::X => RomanNumbers::X,
        }
    }
}
