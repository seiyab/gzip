pub fn deflate(data: &Vec<u8>) -> Vec<u8> {
    let mut block = Vec::<u8>::new();
    block.push(0b00000001u8);
    for b in non_compressed(data).iter() {
        block.push(*b);
    }
    return block;
}

fn non_compressed(data: &Vec<u8>) -> Vec<u8> {
    let mut body = Vec::<u8>::with_capacity(data.len() + 4);
    let len = data.len();
    let len_le = len.to_le_bytes();
    // LEN
    body.push(len_le[0]);
    body.push(len_le[1]);
    // NLEN
    body.push(!len_le[0]);
    body.push(!len_le[1]);
    for b in data.iter() {
        body.push(*b);
    }
    return body;
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
