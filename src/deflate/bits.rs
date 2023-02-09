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
        let mut size = another.size;
        let mut body = another.body;
        while size > 0 {
            let temp_bits: u64 = u64::from(self.bits) + ((body & u64::from(u32::MAX)) << self.i);
            let temp_i = self.i + u8::min(size, 32) as usize;
            body = body.checked_shr(32).unwrap_or(0);
            let bytes = temp_bits.to_le_bytes();
            for b in bytes.into_iter().take(temp_i / 8) {
                self.bytes.push(b);
            }
            self.bits = bytes[temp_i / 8];
            self.i = temp_i % 8;
            size -= u8::min(size, 32);
        }
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

    pub fn last(self) -> Option<u8> {
        assert_eq!(0, self.bytes.len());
        if self.i == 0 {
            None
        } else {
            Some(self.bits)
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShortBits {
    body: u64,
    size: u8,
}

impl ShortBits {
    pub fn code(rev_body: u64, size: u8) -> Self {
        let body = rev_body.reverse_bits().rotate_left(size.into());
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

        let mut bits = Bits::new();
        bits.append(&ShortBits::data(0b_0101, 4));
        bits.append(&ShortBits::data(0b_111, 3));
        bits.append(&ShortBits::data(0b_000, 3));
        bits.append(&ShortBits::data(0b_001101, 6));
        bits.append(&ShortBits::data(0b_10101, 5));
        bits.append(&ShortBits::data(0b_11, 2));
        bits.append(&ShortBits::data(0b_1100, 2));
        let (bytes, _) = bits.drain_bytes();

        assert_eq!(vec![0b0_111_0101, 0b001101_00, 0b0_11_10101], bytes);
    }
}
