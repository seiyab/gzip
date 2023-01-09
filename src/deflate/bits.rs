pub struct Bits {
    bytes: Vec<u8>,
    bits: u8,
    i: usize,
}

impl Bits {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            bits: 0,
            i: 0,
        }
    }

    pub fn add<I: Iterator<Item = bool>>(&mut self, bits: I) {
        for b in bits {
            if b {
                self.bits = self.bits + (1 << self.i);
            }
            self.i = self.i + 1;
            if self.i == 8 {
                self.bytes.push(self.bits);
                self.i = 0;
                self.bits = 0;
            }
        }
    }

    pub fn extend(&mut self, another: &Self) {
        for byte in another.bytes.iter() {
            self.bytes.push(self.bits + (byte << self.i));
            self.bits = byte >> (8 - self.i);
        }
        self.add((0..another.i).map(|i| (another.bits >> i) & 1 > 0));
    }

    pub fn append(&mut self, another: &ShortBits) {
        self.add(another.bits().into_iter());
    }

    pub fn drain_bytes(self) -> (Vec<u8>, Self) {
        let bytes = self.bytes;
        return (
            bytes,
            Self {
                bytes: Vec::new(),
                bits: self.bits,
                i: self.i,
            },
        );
    }

    pub fn last(self) -> u8 {
        assert_eq!(0, self.bytes.len());
        self.bits
    }
}

#[derive(Clone)]
pub struct ShortBits {
    body: u64,
    size: u8,
}

impl ShortBits {
    pub fn code(body: u64, size: u8) -> Self {
        Self { body, size }
    }

    pub fn data(rev_body: u64, size: u8) -> Self {
        let mut body: u64 = 0;
        for i in 0..size {
            body = body << 1;
            body += (rev_body >> i) & 1
        }
        Self { body, size }
    }

    pub fn zero() -> Self {
        Self { body: 0, size: 0 }
    }

    pub fn bits(&self) -> Vec<bool> {
        (0..self.size)
            .map(|i| ((self.body >> i) & 1) > 0)
            .rev()
            .collect()
    }

    pub fn concat(&self, another: &Self) -> Self {
        debug_assert!(u32::from(self.size + another.size) <= u64::BITS);
        Self {
            body: (self.body << another.size) + another.body,
            size: self.size + another.size,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{Bits, ShortBits};

    #[test]
    fn append_short_bits_into_bits() {
        let mut bits = Bits::new();
        bits.append(&ShortBits::data(0x12_34, 2 * 8));
        let (bytes, _) = bits.drain_bytes();

        assert_eq!(vec![0x34, 0x12], bytes);

        let mut bits = Bits::new();
        bits.append(&ShortBits::data(0x06_34_12, 2 * 8 + 4));
        bits.append(&ShortBits::data(0x07_85, 8 + 4));
        let (bytes, _) = bits.drain_bytes();

        assert_eq!(vec![0x12, 0x34, 0x56, 0x78], bytes);
    }
}
