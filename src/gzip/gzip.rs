use std::io::{Read, Write};

use super::crc::Crc;
use crate::deflate::deflate;
use chrono::{DateTime, TimeZone};

pub fn gzip<Tz: TimeZone, R: Read, W: Write>(output: W, mtime: &DateTime<Tz>, input: R) {
    member(output, mtime, input)
}

fn member<Tz: TimeZone, R: Read, W: Write>(mut output: W, mtime: &DateTime<Tz>, mut input: R) {
    output.write_all(&header(mtime)).unwrap();
    let mut crc = Crc::new();
    let mut buf = Vec::new();
    input.read_to_end(&mut buf).unwrap();
    crc = crc.append(&buf);
    output.write_all(&deflate(&buf)).unwrap();
    output.write_all(&crc.get()).unwrap();
    output
        .write_all(
            &buf.len()
                .to_le_bytes()
                .into_iter()
                .take(4)
                .collect::<Vec<_>>(),
        )
        .unwrap();
}

fn header<Tz: TimeZone>(mtime: &DateTime<Tz>) -> Vec<u8> {
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
    use super::gzip;
    use chrono::{DateTime, TimeZone, Utc};
    use flate2::read::GzDecoder;
    use std::io::{BufReader, BufWriter, Read};

    #[test]
    fn read_gzip() {
        let data = "foobar".as_bytes().to_vec();
        let result = gzip_buf(&data, &DateTime::<Utc>::default());
        let mut gunzipper = GzDecoder::new(&result[..]);
        let mut s = String::new();
        if let Err(e) = gunzipper.read_to_string(&mut s) {
            panic!("{e:#?}")
        }

        assert_eq!("foobar", s);
    }

    #[test]
    fn read_gzip_with_duplicated_sequence() {
        let data = "foobar123foo1234foobar".as_bytes().to_vec();
        let result = gzip_buf(&data, &DateTime::<Utc>::default());
        let mut gunzipper = GzDecoder::new(&result[..]);
        let mut s = String::new();
        if let Err(e) = gunzipper.read_to_string(&mut s) {
            panic!("{e:#?}")
        }

        assert_eq!("foobar123foo1234foobar", s);
    }

    fn gzip_buf<Tz: TimeZone>(input: &[u8], mtime: &DateTime<Tz>) -> Vec<u8> {
        let mut out = Vec::new();
        gzip(BufWriter::new(&mut out), mtime, BufReader::new(input));
        println!("{}", out.len());
        out
    }
}
