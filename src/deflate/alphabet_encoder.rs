use super::bits::ShortBits;

pub struct AlphabetEncoder {
    table: Vec<ShortBits>,
}

impl AlphabetEncoder {
    pub fn new(table: Vec<ShortBits>) -> Self {
        Self { table }
    }

    pub fn encode(&self, alphabet: usize) -> ShortBits {
        self.table[alphabet].clone()
    }
}
