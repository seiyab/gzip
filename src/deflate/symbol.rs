use super::alphabet_encoder::AlphabetEncoder;

#[derive(Debug)]
pub enum Symbol {
    Literal(u8),
    EndOfBlock,
    Reference { length: usize, distance: usize },
}

impl Symbol {
    pub const MAX_LENGTH: usize = 257;
    pub const MAX_DISTANCE: usize = 20_000;

    pub fn encode(&self, alphabet_encoder: &AlphabetEncoder) -> Vec<bool> {
        match self {
            &Symbol::Literal(l) => alphabet_encoder.encode(l as usize).bits(),
            &Symbol::EndOfBlock => alphabet_encoder.encode(256).bits(),
            &Symbol::Reference { length, distance } => {
                let mut bits: Vec<bool> = Vec::new();
                bits.extend(length_code(length, alphabet_encoder));
                bits.extend(distance_code(distance));
                bits
            }
        }
    }
}

fn usize_to_bits(x: usize, len: usize) -> Vec<bool> {
    (0..len).map(|i| ((x >> i) & 1) > 0).rev().collect()
}

fn u32_to_bits(x: u32, len: u32) -> Vec<bool> {
    (0..len).map(|i| ((x >> i) & 1) > 0).rev().collect()
}

fn u32_to_rev_bits(x: u32, len: u32) -> Vec<bool> {
    (0..len).map(|i| ((x >> i) & 1) > 0).collect()
}

fn length_code(l: usize, alphabet_encoder: &AlphabetEncoder) -> Vec<bool> {
    if l < 11 {
        return alphabet_encoder.encode(257 - 3 + l).bits();
    }
    if l > Symbol::MAX_LENGTH {
        panic!("unsupported length")
    }
    let extra_bits_len = 64 - (l - 3).leading_zeros() - 3;
    let group_min_length = (1u32 << (extra_bits_len + 2)) + 3;
    let group_min_code = 261 + (extra_bits_len * 4);
    let size_in_group = l as u32 - group_min_length;
    let code = group_min_code + (size_in_group >> extra_bits_len);
    let mut bits = alphabet_encoder.encode(code as usize).bits();
    let extra_bits = u32_to_rev_bits(size_in_group, extra_bits_len);
    bits.extend(extra_bits);
    return bits;
}

fn distance_code(d: usize) -> Vec<bool> {
    if d == 0 {
        panic!("unsupported distance")
    }
    if d < 5 {
        return usize_to_bits(d - 1, 5);
    }
    if d <= Symbol::MAX_DISTANCE {
        let extra_bits_len = 64 - (d - 1).leading_zeros() - 2;
        let group_min_distance = (1u32 << (extra_bits_len + 1)) + 1;
        let group_min_code = 2 + (extra_bits_len * 2);
        let size_in_group = d as u32 - group_min_distance;
        let code = group_min_code + (size_in_group >> extra_bits_len);
        let mut bits = u32_to_bits(code, 5);
        let extra_bits = u32_to_rev_bits(size_in_group, extra_bits_len);
        bits.extend(extra_bits);
        return bits;
    }
    panic!("unsupported distance")
}
