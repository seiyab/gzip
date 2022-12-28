use crate::deflate::bits::ShortBits;

#[derive(Debug, Clone)]
pub enum CodeLengthSymbol {
    Literal(u8),
    CopyPrevious(usize),
    RepeatZero(usize),
}

impl CodeLengthSymbol {
    pub fn code(&self) -> usize {
        match self {
            &Self::Literal(length) => length as usize,
            Self::CopyPrevious(_) => 16usize,
            &Self::RepeatZero(length) => {
                if length < 11 {
                    17usize
                } else if length < 139 {
                    18usize
                } else {
                    panic!("unexpected length")
                }
            }
        }
    }

    pub fn additional_bits(&self) -> ShortBits {
        match self {
            Self::Literal(_) => ShortBits::zero(),
            &Self::CopyPrevious(length) => ShortBits::data(length as u64 - 3, 2),
            &Self::RepeatZero(length) => {
                if length < 11 {
                    ShortBits::data(length as u64 - 3, 3)
                } else if length < 139 {
                    ShortBits::data(length as u64 - 11, 7)
                } else {
                    panic!("unexpected length")
                }
            }
        }
    }
}
