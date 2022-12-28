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

    pub fn as_bytes(mut self) -> Vec<u8> {
        if self.i == 0 {
            return self.bytes;
        }
        self.bytes.push(self.bits);
        return self.bytes;
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
        Self {
            body: (self.body << another.size) + another.body,
            size: self.size + another.size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Bits;
    const O: bool = false;
    const L: bool = true;

    #[test]
    fn bits_add() {
        let mut b = Bits::new();
        b.add([L].iter().copied());
        assert_eq!(vec![1], b.as_bytes());

        let mut b = Bits::new();
        b.add(
            [
                [L, O, O, O, O, O, O, O],
                [O, L, O, O, O, O, O, O],
                [O, O, O, O, L, O, O, O],
            ]
            .concat()
            .iter()
            .copied(),
        );
        assert_eq!(vec![1, 2, 16], b.as_bytes());

        let mut b = Bits::new();
        b.add(
            [L; 3]
                .iter()
                .copied()
                .chain([O; 2].iter().copied())
                .chain([L; 5].iter().copied()),
        );
        assert_eq!(vec![0b11100111, 0b11], b.as_bytes());
    }
}
