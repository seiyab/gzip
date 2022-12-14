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
            self.bits = byte.checked_shr(8 - self.i as u32).unwrap_or(0);
        }
        self.add((0..another.i).map(|i| (another.bits >> i) & 1 > 0));
    }

    pub fn append(&mut self, another: &ShortBits) {
        let bytes = another.body.to_le_bytes();
        let len = (u32::from(another.size) / u8::BITS) as usize;
        let effective_bytes = bytes.iter().take(len);
        for b in effective_bytes {
            self.bytes.push(self.bits + (b << self.i));
            self.bits = b.checked_shr(8 - self.i as u32).unwrap_or(0);
        }
        let rest = bytes[len];
        self.add((0..(u32::from(another.size) % u8::BITS)).map(|i| (rest >> i) & 1 > 0));
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

#[derive(Clone, Debug)]
pub struct ShortBits {
    body: u64,
    size: u8,
}

impl ShortBits {
    pub fn code(rev_body: u64, size: u8) -> Self {
        let mut body: u64 = 0;
        for i in 0..size {
            body = body << 1;
            body += (rev_body >> i) & 1
        }
        Self { body, size }
    }

    pub fn data(body: u64, size: u8) -> Self {
        Self { body, size }
    }

    pub fn trim(self) -> Self {
        Self {
            body: self.body % (1 << self.size),
            size: self.size,
        }
    }

    pub const fn zero() -> Self {
        Self { body: 0, size: 0 }
    }

    pub fn concat(&self, another: &Self) -> Self {
        debug_assert!(u32::from(self.size + another.size) <= u64::BITS);
        Self {
            body: (another.body << self.size) + self.body,
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

        let mut bits = Bits::new();
        bits.append(&ShortBits::data(0x_03_21, 8 + 4));
        bits.append(&ShortBits::data(0x_04, 4));
        bits.append(&ShortBits::data(0x_05, 4));
        bits.append(&ShortBits::data(0x_06, 4));
        bits.append(&ShortBits::data(0x_07, 4));
        bits.append(&ShortBits::data(0x_08, 4));
        let (bytes, _) = bits.drain_bytes();

        assert_eq!(vec![0x21, 0x43, 0x65, 0x87], bytes);
    }
}
