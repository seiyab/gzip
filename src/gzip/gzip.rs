use super::crc;
use crate::deflate::deflate;
use chrono::{DateTime, TimeZone};

pub fn gzip<Tz: TimeZone>(data: &Vec<u8>, mtime: &DateTime<Tz>) -> Vec<u8> {
    member(data, mtime)
}

fn member<Tz: TimeZone>(data: &Vec<u8>, mtime: &DateTime<Tz>) -> Vec<u8> {
    let mut m = header(mtime);
    m.extend(deflate(&data));
    m.extend(crc::crc(&data));
    m.extend(data.len().to_le_bytes().iter().take(4));
    return m;
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
    use chrono::{DateTime, Utc};
    use flate2::read::GzDecoder;
    use std::io::Read;

    #[test]
    fn read_gzip() {
        let data = "foobar".as_bytes().to_vec();
        let result = gzip(&data, &DateTime::<Utc>::default());
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
        let result = gzip(&data, &DateTime::<Utc>::default());
        let mut gunzipper = GzDecoder::new(&result[..]);
        let mut s = String::new();
        if let Err(e) = gunzipper.read_to_string(&mut s) {
            panic!("{e:#?}")
        }

        assert_eq!("foobar123foo1234foobar", s);
    }
}
