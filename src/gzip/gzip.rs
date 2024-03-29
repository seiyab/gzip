use std::io::{Read, Write};

use super::{checksum::Checksum, Config};
use crate::deflate::deflate;
use chrono::{DateTime, Local};

pub fn gzip<R: Read, W: Write>(output: W, input: R, cfg: Config) {
    member(output, input, cfg)
}

fn member<R: Read, W: Write>(mut output: W, input: R, cfg: Config) {
    output.write_all(&header(&cfg.mtime)).unwrap();
    let mut input = Checksum::new(input);
    deflate(&mut output, &mut input, cfg.buf_size);
    output.write_all(&input.crc_bytes()).unwrap();
    output.write_all(&input.isize_bytes()).unwrap();
}

fn header(mtime: &DateTime<Local>) -> Vec<u8> {
    let mut h = Vec::<u8>::new();
    h.push(ID1);
    h.push(ID2);
    h.push(CM);
    h.push(Flg {}.byte());
    for b in mtime.timestamp().to_le_bytes().iter().take(4) {
        h.push(*b);
    }
    h.push(XFL);
    h.push(OS_UNKNOWN);

    return h;
}

const ID1: u8 = 0x1f;
const ID2: u8 = 0x8b;
const CM: u8 = 0x08;
const XFL: u8 = 0x0;
const OS_UNKNOWN: u8 = 0xff;

struct Flg {}

impl Flg {
    fn byte(&self) -> u8 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::gzip::config::Config;

    use super::gzip;
    use chrono::DateTime;
    use flate2::read::GzDecoder;
    use std::io::{BufReader, BufWriter, Read};

    #[test]
    fn read_gzip() {
        let buf_sizes = [1024, 4, 8];
        let inputs = ["foobar", "foobar123foo1234foobar"];
        for buf_size in buf_sizes.into_iter() {
            for input in inputs.into_iter() {
                let data = input.as_bytes().to_vec();
                let result = gzip_buf(&data, buf_size);
                let mut gunzipper = GzDecoder::new(&result[..]);
                let mut s = String::new();
                if let Err(e) = gunzipper.read_to_string(&mut s) {
                    panic!("input: {input}: {e:#?}")
                }

                assert_eq!(input, s);
            }
        }
    }

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
            let result = gzip_buf(&data, 10_000);
            let mut gunzipper = GzDecoder::new(&result[..]);
            let mut s = String::new();
            if let Err(e) = gunzipper.read_to_string(&mut s) {
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
            let result = gzip_buf(&data, 10_000);
            let mut gunzipper = GzDecoder::new(&result[..]);
            let mut s = String::new();
            if let Err(e) = gunzipper.read_to_string(&mut s) {
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

    fn gzip_buf(input: &[u8], buf_size: usize) -> Vec<u8> {
        let mut out = Vec::new();
        gzip(
            BufWriter::new(&mut out),
            BufReader::new(input),
            cfg(buf_size),
        );
        out
    }

    fn cfg(buf_size: usize) -> Config {
        Config {
            mtime: DateTime::default(),
            buf_size,
        }
    }
}
