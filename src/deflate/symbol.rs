use super::{alphabet_encoder::AlphabetEncoder, bits::ShortBits};

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
        alphabet_encoder
            .encode(self.code())
            .concat(&self.additional_bits())
            .bits()
    }

    pub fn code(&self) -> usize {
        match self {
            &Symbol::Literal(l) => l as usize,
            &Symbol::EndOfBlock => 256,
            &Symbol::Reference {
                length,
                distance: _,
            } => {
                if length < 11 {
                    return 257 - 3 + length;
                }
                if length > Symbol::MAX_LENGTH {
                    panic!("unsupported length")
                }
                let extra_bits_len = 64 - (length - 3).leading_zeros() - 3;
                let group_min_length = (1u32 << (extra_bits_len + 2)) + 3;
                let group_min_code = 261 + (extra_bits_len * 4);
                let size_in_group = length as u32 - group_min_length;
                let code = group_min_code + (size_in_group >> extra_bits_len);
                code as usize
            }
        }
    }

    pub fn additional_bits(&self) -> ShortBits {
        match self {
            &Symbol::Literal(_) => ShortBits::new(0, 0),
            &Symbol::EndOfBlock => ShortBits::new(0, 0),
            &Symbol::Reference { length, distance } => {
                length_extra_bits(length).concat(&distance_bits(distance))
            }
        }
    }
}

fn length_extra_bits(l: usize) -> ShortBits {
    if l < 11 {
        return ShortBits::new(0, 0);
    }
    if l > Symbol::MAX_LENGTH {
        panic!("unsupported length")
    }
    let extra_bits_len = 64u32 - (l - 3).leading_zeros() - 3;
    let group_min_length = (1u32 << (extra_bits_len + 2)) + 3;
    let size_in_group = l as u32 - group_min_length;
    return ShortBits::rev(size_in_group.into(), extra_bits_len as u8);
}

fn distance_bits(d: usize) -> ShortBits {
    if d == 0 {
        panic!("unsupported distance")
    }
    if d < 5 {
        return ShortBits::new((d - 1) as u64, 5);
    }
    if d <= Symbol::MAX_DISTANCE {
        let extra_bits_len = 64 - (d - 1).leading_zeros() - 2;
        let group_min_distance = (1u32 << (extra_bits_len + 1)) + 1;
        let group_min_code = 2 + (extra_bits_len * 2);
        let size_in_group = d as u32 - group_min_distance;
        let code = group_min_code + (size_in_group >> extra_bits_len);
        let code_bits = ShortBits::new(code.into(), 5);
        let extra_bits = ShortBits::rev(size_in_group.into(), extra_bits_len as u8);
        return code_bits.concat(&extra_bits);
    }
    panic!("unsupported distance")
}
