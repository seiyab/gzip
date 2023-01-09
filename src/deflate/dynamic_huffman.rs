use super::bits::Bits;
use super::code_length_table::CodeLengthTable;
use super::symbolize;

pub fn dynamic_huffman(input: &[u8], output: Bits) -> Bits {
    let mut bits = output;
    bits.add([false, false, true].iter().copied());
    let symbols = symbolize(input);
    let mut lit_weights = vec![0; 286];
    for s in symbols.iter() {
        lit_weights[s.code()] += 1;
    }
    let lit_table = CodeLengthTable::analyze(&lit_weights, 15);
    let mut dist_weights = vec![0; 30];
    for s in symbols.iter() {
        if let Some(c) = s.dist_code() {
            dist_weights[c] += 1;
        }
    }
    let dist_table = CodeLengthTable::analyze(&dist_weights, 15);
    bits.extend(&CodeLengthTable::encode(&lit_table, &dist_table));

    for s in symbols.iter() {
        bits.append(&s.encode(&lit_table.build_encoder(), &dist_table.build_encoder()));
    }
    return bits;
}

#[cfg(test)]
mod tests {
    use crate::deflate::bits::{Bits, ShortBits};

    use super::dynamic_huffman;
    use flate2::read::DeflateDecoder;
    use std::io::Read;

    #[test]
    fn literal_tests() {
        let cases = [
            "foobar",
            "foobar123foobar4foobar4xyz",
            "0123456789_0123456789",
            "this is a",
        ];
        for input in cases.into_iter() {
            let data = input.as_bytes().to_vec();
            let result = deflate(&data);
            let mut inflator = DeflateDecoder::new(&result[..]);
            let mut s = String::new();
            if let Err(e) = inflator.read_to_string(&mut s) {
                panic!("{e:#?}")
            }

            assert_eq!(input, s);
        }
    }

    #[test]
    fn repeat_tests() {
        let three_times = (5..=10).chain([15, 25, 50]).map(|l| (l, 3));
        let thousand_times = (1..=4).map(|l| (l, 1000));
        for (l, r) in three_times.chain(thousand_times) {
            let value = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"[..l].repeat(r);
            let data = value.as_bytes().to_vec();
            let result = deflate(&data);
            let mut inflator = DeflateDecoder::new(&result[..]);
            let mut s = String::new();
            if let Err(e) = inflator.read_to_string(&mut s) {
                panic!("error: {e:#?}, length: {l}, repeat: {r}")
            }

            if data.len() < 20 {
                assert_eq!(value, s);
            } else {
                let actual_omitted = &s[..20];
                assert_eq!(
                    value, s,
                    "length: {l}, repeat: {r}, actual: {actual_omitted}..."
                );
            }
        }
    }

    #[test]
    fn distance_tests() {
        let ds = (3..=15).into_iter().chain([
            16, 17, 19, 24, 32, 33, 50, 64, 65, 90, 128, 200, 400, 800, 1000,
        ]);
        for d in ds {
            let value = format!("abc{}abc{}abc", "-".repeat(d - 3), "-".repeat(d - 3));
            let data = value.as_bytes().to_vec();
            let result = deflate(&data);
            let mut inflator = DeflateDecoder::new(&result[..]);
            let mut s = String::new();
            if let Err(e) = inflator.read_to_string(&mut s) {
                panic!("{e:#?}")
            }

            assert_eq!(value, s, "distance: {d}");
        }
    }

    #[test]
    fn huffman_256_bytes() {
        let data = (0..255u8).collect::<Vec<_>>();
        let result = deflate(&data);
        let mut deflater = DeflateDecoder::new(&result[..]);
        let mut buf = Vec::new();
        if let Err(e) = deflater.read_to_end(&mut buf) {
            panic!("{e:#?}")
        }
        assert_eq!(&data, &buf);
    }

    #[test]
    fn repeated_3_chars_shrinks() {
        let data = "abc".repeat(1000).as_bytes().to_vec();
        let result = deflate(&data);
        assert!(result.len() < data.len());
        let mut deflater = DeflateDecoder::new(&result[..]);
        let mut buf = Vec::new();
        if let Err(e) = deflater.read_to_end(&mut buf) {
            panic!("{e:#?}")
        }
        assert_eq!(&data, &buf);
    }

    fn deflate(data: &[u8]) -> Vec<u8> {
        let bits = dynamic_huffman(data, Bits::new());
        let (mut out, bits) = bits.drain_bytes();
        let (last, rest) = last_block(bits).drain_bytes();
        out.extend(last);
        out.push(rest.last());
        out
    }

    fn last_block(bits: Bits) -> Bits {
        let mut out = bits;
        out.add([true, true, false].iter().copied());
        out.append(&ShortBits::code(0, 7));
        out
    }
}
