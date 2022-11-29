#[derive(Debug)]
pub enum Symbol {
    Literal(u8),
    EndOfBlock,
    Length(usize),
    Distance(usize),
}

impl Symbol {
    pub fn encode(&self) -> Vec<bool> {
        match self {
            &Symbol::Literal(l) => {
                if l <= 143 {
                    let c = 0b00110000 + (l as usize);
                    usize_to_bits(c, 8)
                } else {
                    let c = 0b110010000 - 144 + (l as usize);
                    usize_to_bits(c, 9)
                }
            }
            &Symbol::EndOfBlock => vec![false; 7],
            &Symbol::Length(l) => length_code(l),
            &Symbol::Distance(d) => distance_code(d),
        }
    }
}

fn usize_to_bits(x: usize, len: usize) -> Vec<bool> {
    (0..len).map(|i| ((x >> i) & 1) > 0).rev().collect()
}

fn length_code(l: usize) -> Vec<bool> {
    if l != 3 {
        panic!("unsupported length");
    }
    usize_to_bits(257 - 256, 7)
}

fn distance_code(d: usize) -> Vec<bool> {
    if d != 3 {
        panic!("unsupported distance");
    }
    usize_to_bits(2, 5)
}
