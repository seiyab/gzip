use super::bits::Bits;

pub fn fixed_huffman(data: &Vec<u8>) -> Vec<u8> {
    let mut bits = Bits::new();
    bits.add([true, true, false].iter());
    for &b in data.iter() {
        bits.add(Symbol::Literal(b).encode().iter());
    }
    bits.add(Symbol::EndOfBlock.encode().iter());
    return bits.as_bytes();
}

enum Symbol {
    Literal(u8),
    EndOfBlock,
}

impl Symbol {
    fn encode(&self) -> Vec<bool> {
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
        }
    }
}

fn usize_to_bits(x: usize, len: usize) -> Vec<bool> {
    (0..len).map(|i| ((x >> i) & 1) > 0).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::fixed_huffman;
    use flate2::read::DeflateDecoder;
    use std::io::Read;

    #[test]
    fn fixed_huffman_string() {
        let data = "foobar".as_bytes().to_vec();
        let result = fixed_huffman(&data);
        let mut deflater = DeflateDecoder::new(&result[..]);
        let mut s = String::new();
        if let Err(e) = deflater.read_to_string(&mut s) {
            panic!("{e:#?}")
        }

        assert_eq!("foobar", s);
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
}
