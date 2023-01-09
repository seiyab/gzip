use std::io::Read;

use super::crc::Crc;

pub struct Checksum<R: Read> {
    inner: R,
    crc: Crc,
    isize: usize,
}

impl<R: Read> Checksum<R> {
    pub fn new(read: R) -> Self {
        Self {
            inner: read,
            crc: Crc::new(),
            isize: 0,
        }
    }

    pub fn crc_bytes(&self) -> [u8; 4] {
        self.crc.get()
    }

    pub fn isize_bytes(&self) -> [u8; 4] {
        self.isize
            .to_le_bytes()
            .into_iter()
            .take(4)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

impl<R: Read> Read for Checksum<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let r = self.inner.read(buf);
        if let Ok(s) = r {
            self.crc = self.crc.append(&buf[..s]);
            (self.isize, _) = self.isize.overflowing_add(s)
        }
        r
    }
}
