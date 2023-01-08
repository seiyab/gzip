use super::dynamic_huffman::dynamic_huffman;

pub fn deflate(data: &Vec<u8>) -> Vec<u8> {
    dynamic_huffman(data)
}

#[cfg(test)]
mod tests {
    use super::deflate;
    use flate2::read::DeflateDecoder;
    use std::io::Read;

    #[test]
    fn read_deflate() {
        let data = "foobar".as_bytes().to_vec();
        let result = deflate(&data);
        let mut deflater = DeflateDecoder::new(&result[..]);
        let mut s = String::new();
        if let Err(e) = deflater.read_to_string(&mut s) {
            panic!("{e:#?}")
        }

        assert_eq!("foobar", s);
    }

    #[test]
    fn read_deflate_256_bytes() {
        let data = (0..255u8).collect();
        let result = deflate(&data);
        let mut deflater = DeflateDecoder::new(&result[..]);
        let mut buf = Vec::new();
        if let Err(e) = deflater.read_to_end(&mut buf) {
            panic!("{e:#?}")
        }

        assert_eq!(&data, &buf);
    }
}
