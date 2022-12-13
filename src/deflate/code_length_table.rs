use super::{alphabet_encoder::AlphabetEncoder, bits::ShortBits};

pub struct CodeLengthTable {
    table: Vec<u8>,
}

impl CodeLengthTable {
    pub fn build_encoder(&self) -> AlphabetEncoder {
        let mut entries: Vec<(usize, &u8)> = self.table.iter().enumerate().collect();
        entries.sort_by(|left, right| {
            let len_ord = left.1.cmp(right.1);
            if len_ord != std::cmp::Ordering::Equal {
                return len_ord;
            }
            return left.0.cmp(&right.0);
        });
        let mut table = vec![ShortBits::new(0, 0); 288];

        let mut code = 0u64;
        let mut bits: u8 = 0;
        for &(i, &len) in entries.iter() {
            if len == 0 {
                continue;
            }
            if len > bits {
                code = code << (len - bits);
                bits = len;
            }
            table[i] = ShortBits::new(code, bits);
            code = code + 1;
        }

        return AlphabetEncoder::new(table);
    }
}

impl Default for CodeLengthTable {
    fn default() -> Self {
        Self {
            table: (0..288)
                .map(|i| {
                    if i < 144 {
                        8
                    } else if i < 256 {
                        9
                    } else if i < 280 {
                        7
                    } else {
                        8
                    }
                })
                .collect(),
        }
    }
}
