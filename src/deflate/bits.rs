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

    pub fn add<'a, I: Iterator<Item = &'a bool>>(&mut self, bits: I) {
        for &b in bits {
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

    pub fn as_bytes(mut self) -> Vec<u8> {
        if self.i == 0 {
            return self.bytes;
        }
        self.bytes.push(self.bits);
        return self.bytes;
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
        b.add([L].iter());
        assert_eq!(vec![1], b.as_bytes());

        let mut b = Bits::new();
        b.add(
            [
                [L, O, O, O, O, O, O, O],
                [O, L, O, O, O, O, O, O],
                [O, O, O, O, L, O, O, O],
            ]
            .concat()
            .iter(),
        );
        assert_eq!(vec![1, 2, 16], b.as_bytes());

        let mut b = Bits::new();
        b.add([L; 3].iter().chain([O; 2].iter()).chain([L; 5].iter()));
        assert_eq!(vec![0b11100111, 0b11], b.as_bytes());
    }
}
