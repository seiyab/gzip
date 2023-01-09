use std::io::{BufRead, BufReader, Read, Write};

use super::{bits::Bits, dynamic_huffman::dynamic_huffman};

pub fn deflate<R: Read, W: Write>(mut output: W, input: R, buf_size: usize) {
    let mut reader = BufReader::with_capacity(buf_size, input);
    let mut bits = Bits::new();
    loop {
        let length = {
            let buf = reader.fill_buf().unwrap();
            if buf.len() == 0 {
                break;
            } else {
                let (out, rest) = dynamic_huffman(buf, bits).drain_bytes();
                output.write_all(&out).unwrap();
                bits = rest;
                buf.len()
            }
        };
        reader.consume(length);
    }
    output.write_all(&[bits.last()]).unwrap();
}

#[cfg(test)]
mod tests {
    use super::deflate;
    use flate2::read::DeflateDecoder;
    use std::io::{BufReader, BufWriter, Read};

    #[test]
    fn read_deflate() {
        let data = "foobar".as_bytes().to_vec();
        let result = deflate_buf(&data);
        let mut deflater = DeflateDecoder::new(&result[..]);
        let mut s = String::new();
        if let Err(e) = deflater.read_to_string(&mut s) {
            panic!("{e:#?}")
        }

        assert_eq!("foobar", s);
    }

    #[test]
    fn read_deflate_256_bytes() {
        let data = (0..255u8).collect::<Vec<_>>();
        let result = deflate_buf(&data);
        let mut deflater = DeflateDecoder::new(&result[..]);
        let mut buf = Vec::new();
        if let Err(e) = deflater.read_to_end(&mut buf) {
            panic!("{e:#?}")
        }

        assert_eq!(&data, &buf);
    }

    fn deflate_buf(input: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        deflate(BufWriter::new(&mut out), BufReader::new(input), 1024);
        return out;
    }
}
