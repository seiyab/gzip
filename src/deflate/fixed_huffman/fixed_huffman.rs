use super::{
    locator::{Locator, Progress},
    symbol::Symbol,
};
use crate::deflate::bits::Bits;

pub fn fixed_huffman(data: &Vec<u8>) -> Vec<u8> {
    let mut bits = Bits::new();
    bits.add([true, true, false].iter());
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut locator = Locator::new();
    locator.scan(data, |i, locs| {
        if let Some(loc) = locs.back() {
            let dist = i - loc;
            if dist < 129 {
                symbols.push(Symbol::Length(3));
                symbols.push(Symbol::Distance(dist));
                return Progress(3);
            }
        }
        symbols.push(Symbol::Literal(data[i]));
        return Progress(1);
    });

    symbols.push(Symbol::EndOfBlock);

    for s in symbols.iter() {
        bits.add(s.encode().iter());
    }
    return bits.as_bytes();
}

#[cfg(test)]
mod tests {
    use super::fixed_huffman;
    use flate2::read::DeflateDecoder;
    use std::io::Read;

    macro_rules! fixed_huffman_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let value = $value;
                let data = value.as_bytes().to_vec();
                let result = fixed_huffman(&data);
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

    fixed_huffman_tests! {
        no_duplicate: "foobar",
        with_some_duplicate: "foobar123foobar4foobar4xyz",
        with_duplicate_10_characters: "0123456789_0123456789",
        with_some_duplicate_2: "this is a",
    }

    #[test]
    fn repeat_tests() {
        let three_times = (5..=8).map(|l| (l, 3));
        let thousand_times = (1..=4).map(|l| (l, 1000));
        for (l, r) in three_times.chain(thousand_times) {
            let value = "abcdefghijklmnopqrstuvwxyz"[..l].repeat(r);
            let data = value.as_bytes().to_vec();
            let result = fixed_huffman(&data);
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
        let ds = (3..=15)
            .into_iter()
            .chain([16, 17, 19, 24, 32, 33, 50, 64, 65, 90, 128]);
        for d in ds {
            let value = format!("abc{}abc{}abc", "-".repeat(d - 3), "-".repeat(d - 3));
            let data = value.as_bytes().to_vec();
            let result = fixed_huffman(&data);
            let mut deflator = DeflateDecoder::new(&result[..]);
            let mut s = String::new();
            if let Err(e) = deflator.read_to_string(&mut s) {
                panic!("{e:#?}")
            }

            assert_eq!(value, s, "distance: {d}");
        }
    }

    #[test]
    fn fixed_huffman_256_bytes() {
        let data = (0..255u8).collect();
        let result = fixed_huffman(&data);
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
        let result = fixed_huffman(&data);
        assert!(result.len() < data.len());
        let mut deflater = DeflateDecoder::new(&result[..]);
        let mut buf = Vec::new();
        if let Err(e) = deflater.read_to_end(&mut buf) {
            panic!("{e:#?}")
        }
        assert_eq!(&data, &buf);
    }
}
