#[derive(Debug)]
pub enum Symbol {
    Literal(u8),
    EndOfBlock,
    Length(usize),
    Distance(usize),
}

impl Symbol {
    pub const MAX_LENGTH: usize = 114;

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

fn u32_to_bits(x: u32, len: u32) -> Vec<bool> {
    (0..len).map(|i| ((x >> i) & 1) > 0).rev().collect()
}

fn u32_to_rev_bits(x: u32, len: u32) -> Vec<bool> {
    (0..len).map(|i| ((x >> i) & 1) > 0).collect()
}

fn length_code(l: usize) -> Vec<bool> {
    if l < 11 {
        return usize_to_bits(257 - 3 + l, 7);
    }
    if l > Symbol::MAX_LENGTH {
        panic!("unsupported length")
    }
    let extra_bits_len = 64 - (l - 3).leading_zeros() - 3;
    let group_min_length = (1u32 << (extra_bits_len + 2)) + 3;
    let group_min_code = 261 + (extra_bits_len * 4);
    let size_in_group = l as u32 - group_min_length;
    let code = group_min_code + (size_in_group >> extra_bits_len);
    let mut bits = u32_to_bits(code, 7);
    let extra_bits = u32_to_rev_bits(size_in_group, extra_bits_len);
    bits.extend(extra_bits);
    return bits;
}

fn distance_code(d: usize) -> Vec<bool> {
    if d < 5 {
        distance_bits_for(d, 0, 0, 1)
    } else if d < 9 {
        distance_bits_for(d, 4, 1, 5)
    } else if d < 17 {
        distance_bits_for(d, 6, 2, 9)
    } else if d < 33 {
        distance_bits_for(d, 8, 3, 17)
    } else if d < 65 {
        distance_bits_for(d, 10, 4, 33)
    } else if d < 129 {
        distance_bits_for(d, 12, 5, 65)
    } else {
        panic!("unsupported distance")
    }
}

fn distance_bits_for(
    dist: usize,
    group_first_code: usize,
    group_bits: u32,
    group_first_dist: usize,
) -> Vec<bool> {
    let group_code_size = 2usize.pow(group_bits);
    let degree_in_group = dist - group_first_dist;
    let code = group_first_code + degree_in_group / group_code_size;
    let mut bits = usize_to_bits(code, 5);
    let mut extra = usize_to_bits(degree_in_group, group_bits as usize);
    extra.reverse();
    bits.extend(extra);
    bits
}
