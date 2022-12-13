use super::{
    code_length_table::CodeLengthTable,
    locator::{Locator, Progress},
    symbol::Symbol,
};
use crate::deflate::bits::Bits;

pub fn huffman(data: &Vec<u8>) -> Vec<u8> {
    let mut bits = Bits::new();
    bits.add([true, true, false].iter());
    let encoder = CodeLengthTable::default().build_encoder();

    for s in symbolize(data).iter() {
        bits.add(s.encode(&encoder).iter());
    }
    return bits.as_bytes();
}

fn symbolize(data: &Vec<u8>) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut locator = Locator::new();
    locator.scan(data, |i, locs| {
        let (length, distance) =
            longest_duplicate(data, i, locs.into_iter().rev().take(10).copied());
        if length >= 3 {
            symbols.push(Symbol::Reference { length, distance });
            return Progress(length);
        }
        symbols.push(Symbol::Literal(data[i]));
        return Progress(1);
    });

    symbols.push(Symbol::EndOfBlock);

    return symbols;
}

fn longest_duplicate<I: Iterator<Item = usize>>(
    data: &Vec<u8>,
    i: usize,
    refs: I,
) -> (usize, usize) {
    let mut len = 0;
    let mut distance = 0;
    for loc in refs {
        let dist_candidate = i - loc;
        if dist_candidate >= Symbol::MAX_DISTANCE {
            continue;
        }
        let len_candidate = duplicate_length(data, i, loc);
        if len_candidate > len {
            (len, distance) = (len_candidate, dist_candidate);
        }
    }
    return (len, distance);
}

fn duplicate_length(data: &Vec<u8>, i: usize, j: usize) -> usize {
    let mut len = 0;
    loop {
        if len >= Symbol::MAX_LENGTH {
            break;
        }
        match (data.get(i + len), data.get(j + len)) {
            (Some(&x), Some(&y)) => {
                if x != y {
                    break;
                }
            }
            _ => break,
        }
        len += 1;
    }
    return len;
}

#[cfg(test)]
mod tests {
    use super::huffman;
    use flate2::read::DeflateDecoder;
    use std::io::Read;

    macro_rules! huffman_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let value = $value;
                let data = value.as_bytes().to_vec();
                let result = huffman(&data);
                let mut deflator = DeflateDecoder::new(&result[..]);
                let mut s = String::new();
                if let Err(e) = deflator.read_to_string(&mut s) {
                    panic!("{e:#?}")
                }

                assert_eq!(value, s);
            }
        )*
        }
    }

    huffman_tests! {
        no_duplicate: "foobar",
        with_some_duplicate: "foobar123foobar4foobar4xyz",
        with_duplicate_10_characters: "0123456789_0123456789",
        with_some_duplicate_2: "this is a",
    }

    #[test]
    fn repeat_tests() {
        let three_times = (5..=10).chain([15, 25, 50]).map(|l| (l, 3));
        let thousand_times = (1..=4).map(|l| (l, 1000));
        for (l, r) in three_times.chain(thousand_times) {
            let value = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"[..l].repeat(r);
            let data = value.as_bytes().to_vec();
            let result = huffman(&data);
            let mut deflator = DeflateDecoder::new(&result[..]);
            let mut s = String::new();
            if let Err(e) = deflator.read_to_string(&mut s) {
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
            let result = huffman(&data);
            let mut deflator = DeflateDecoder::new(&result[..]);
            let mut s = String::new();
            if let Err(e) = deflator.read_to_string(&mut s) {
                panic!("{e:#?}")
            }

            assert_eq!(value, s, "distance: {d}");
        }
    }

    #[test]
    fn huffman_256_bytes() {
        let data = (0..255u8).collect();
        let result = huffman(&data);
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
        let result = huffman(&data);
        assert!(result.len() < data.len());
        let mut deflater = DeflateDecoder::new(&result[..]);
        let mut buf = Vec::new();
        if let Err(e) = deflater.read_to_end(&mut buf) {
            panic!("{e:#?}")
        }
        assert_eq!(&data, &buf);
    }
}
