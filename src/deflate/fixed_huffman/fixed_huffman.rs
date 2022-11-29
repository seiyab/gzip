use super::symbol::Symbol;
use crate::deflate::bits::Bits;

pub fn fixed_huffman(data: &Vec<u8>) -> Vec<u8> {
    let mut bits = Bits::new();
    bits.add([true, true, false].iter());
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut i = 0;
    loop {
        if i >= data.len() {
            break;
        }
        let b = data[i];
        let m = if i >= 3 && i + 2 < data.len() {
            data[i] == data[i - 3] && data[i + 1] == data[i - 2] && data[i + 2] == data[i - 1]
        } else {
            false
        };
        if m {
            symbols.push(Symbol::Length(3));
            symbols.push(Symbol::Distance(3));
            i += 3;
        } else {
            symbols.push(Symbol::Literal(b));
            i += 1;
        }
    }
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
                let data = $value.as_bytes().to_vec();
                let result = fixed_huffman(&data);
                let mut deflater = DeflateDecoder::new(&result[..]);
                let mut s = String::new();
                if let Err(e) = deflater.read_to_string(&mut s) {
                    panic!("{e:#?}")
                }

                assert_eq!($value, s);
            }
        )*
        }
    }

    fixed_huffman_tests! {
        no_duplicate: "foobar",
        with_some_duplicate: "foobar123foobar4foobar4xyz",
        with_duplicate_10_characters: "0123456789_0123456789",
        with_some_duplicate_2: "this is a",
        repeat_1x2000: "a".repeat(2000),
        repeat_2x1000: "ab".repeat(1000),
        repeat_3x1000: "abc".repeat(1000),
        repeat_4x1000: "abcd".repeat(1000),
        repeat_5x1000: "abcde".repeat(1000),
        repeat_6x100: "abcdef".repeat(100),
        repeat_10x100: "abcdefghij".repeat(100),
        repeat_10x1000: "abcdefghij".repeat(1000),
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
