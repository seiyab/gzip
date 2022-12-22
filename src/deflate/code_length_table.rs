use std::{cmp::Ordering, mem::size_of, ops::Add};

use super::{
    alphabet_encoder::AlphabetEncoder,
    bits::{Bits, ShortBits},
};

pub struct CodeLengthTable {
    table: Vec<u8>,
}

impl CodeLengthTable {
    pub fn analyze(weights: &Vec<u64>, max_length: u8) -> Self {
        let mut stat: Vec<(usize, u64)> = weights
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, w)| w > 0)
            .collect();
        stat.sort_by(|l, r| match l.1.cmp(&r.1) {
            Ordering::Equal => l.0.cmp(&r.0),
            x => x.reverse(),
        });

        let mut table = vec![0u8; weights.len()];
        for &(i, len) in Self::decide_code_lengths(&stat, max_length, 0).iter() {
            table[i] = len;
        }
        return Self { table };
    }

    fn decide_code_lengths(
        stat: &Vec<(usize, u64)>,
        max_length: u8,
        depth: u8,
    ) -> Vec<(usize, u8)> {
        if stat.len() <= 1 {
            return stat.iter().map(|(i, _)| (*i, depth)).collect();
        }
        if depth > max_length {
            panic!("failed to calculate code length (depth {depth} > max_length {max_length})");
        }
        let total_weight = stat.iter().map(|(_, w)| *w).fold(0, Add::add);
        let cap = 1 << (max_length - depth);
        let side_max_cap = cap / 2;
        let mut left_weight = stat[0].1;
        let mut i = 1usize;
        for &(_, w) in stat[1..].iter() {
            if stat.len() - i >= side_max_cap {
                continue;
            }
            if i >= side_max_cap {
                break;
            }
            if left_weight + w / 2 >= total_weight / 2 {
                break;
            }
            left_weight += w;
            i += 1;
        }

        let mut ret = Self::decide_code_lengths(&stat[..i].to_vec(), max_length, depth + 1);
        ret.extend(Self::decide_code_lengths(
            &stat[i..].to_vec(),
            max_length,
            depth + 1,
        ));
        return ret;
    }

    pub fn flat(size: usize) -> Self {
        let longer_length: u8 = (size_of::<usize>() as u8) * 8 - size.leading_zeros() as u8;
        let ll_cap: usize = 1usize << longer_length;
        let shorter_num = ll_cap - size;
        let longer_num = size - shorter_num;
        let mut table = vec![longer_length - 1; shorter_num];
        table.extend(vec![longer_length; longer_num]);
        return Self { table };
    }

    pub fn encode(lit_table: &Self, dist_table: &Self) -> Bits {
        let mut bits = Bits::new();
        let hlit = ShortBits::data(lit_table.table.len() as u64 - 257, 5);
        let hdist = ShortBits::data(dist_table.table.len() as u64 - 1, 5);
        let hclen = ShortBits::data(19 - 4, 4);
        bits.add(hlit.bits().iter().copied());
        bits.add(hdist.bits().iter().copied());
        bits.add(hclen.bits().iter().copied());

        let lc_table = CodeLengthTable::flat(19);
        let lc_encoder = lc_table.build_encoder();
        for &cl in CODE_LENGTH_ORDER.iter() {
            let l = lc_table.table[cl];
            bits.add(ShortBits::data(l.into(), 3).bits().iter().copied());
        }
        for &l in lit_table.table.iter() {
            bits.add(lc_encoder.encode(l as usize).bits().iter().copied());
        }
        for &l in dist_table.table.iter() {
            bits.add(lc_encoder.encode(l as usize).bits().iter().copied());
        }
        return bits;
    }

    pub fn build_encoder(&self) -> AlphabetEncoder {
        let mut entries: Vec<(usize, &u8)> = self.table.iter().enumerate().collect();
        entries.sort_by(|left, right| {
            let len_ord = left.1.cmp(right.1);
            if len_ord != std::cmp::Ordering::Equal {
                return len_ord;
            }
            return left.0.cmp(&right.0);
        });
        let mut table = vec![ShortBits::code(0, 0); 288];

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
            table[i] = ShortBits::code(code, bits);
            code = code + 1;
        }

        return AlphabetEncoder::new(table);
    }
}

const CODE_LENGTH_ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];
