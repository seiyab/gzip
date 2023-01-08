use super::{
    locator::{Locator, Progress},
    symbol::Symbol,
};

pub fn symbolize(data: &Vec<u8>) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut locator = Locator::new();
    locator.scan(data, |i, locs| {
        let (length, distance) =
            longest_duplicate(data, i, locs.into_iter().rev().take(10).copied());
        if length >= 3 {
            symbols.push(Symbol::Reference { length, distance });
            return Progress(length);
        }
        symbols.push(Symbol::Literal(data[i]));
        return Progress(1);
    });

    symbols.push(Symbol::EndOfBlock);

    return symbols;
}

fn longest_duplicate<I: Iterator<Item = usize>>(
    data: &Vec<u8>,
    i: usize,
    refs: I,
) -> (usize, usize) {
    let mut len = 0;
    let mut distance = 0;
    for loc in refs {
        let dist_candidate = i - loc;
        if dist_candidate >= Symbol::MAX_DISTANCE {
            continue;
        }
        let len_candidate = duplicate_length(data, i, loc);
        if len_candidate > len {
            (len, distance) = (len_candidate, dist_candidate);
        }
    }
    return (len, distance);
}

fn duplicate_length(data: &Vec<u8>, i: usize, j: usize) -> usize {
    let mut len = 0;
    loop {
        if len >= Symbol::MAX_LENGTH {
            break;
        }
        match (data.get(i + len), data.get(j + len)) {
            (Some(&x), Some(&y)) => {
                if x != y {
                    break;
                }
            }
            _ => break,
        }
        len += 1;
    }
    return len;
}
