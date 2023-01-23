use super::{locator::Locator, symbol::Symbol};

pub fn symbolize(data: &[u8]) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut locator = Locator::new();
    let mut cursor = 0usize;
    if let Some(&b) = data.get(0) {
        locator.slide_hash(b);
    }
    if let Some(&b) = data.get(1) {
        locator.slide_hash(b);
    }
    for i in 0..data.len() {
        let hash = if let Some(&nx_nx) = data.get(i + 2) {
            Some(locator.slide_hash(nx_nx))
        } else {
            None
        };
        if cursor > i {
            if let Some(h) = hash {
                locator.register(h, i);
            }
            continue;
        }
        let new_symbol = {
            let locs = hash.and_then(|h| locator.locate(h));
            match locs {
                None => Symbol::Literal(data[i]),
                Some(locs) => {
                    let (length, distance) =
                        longest_duplicate(data, i, locs.into_iter().rev().take(30).copied());
                    if length >= 3 {
                        Symbol::Reference { length, distance }
                    } else {
                        Symbol::Literal(data[i])
                    }
                }
            }
        };
        cursor += match &new_symbol {
            Symbol::Literal(_) => 1,
            Symbol::Reference {
                length,
                distance: _,
            } => *length,
            _ => 0,
        };
        symbols.push(new_symbol);
        if let Some(h) = hash {
            locator.register(h, i);
        }
    }
    symbols.push(Symbol::EndOfBlock);

    return symbols;
}

fn longest_duplicate<I: Iterator<Item = usize>>(data: &[u8], i: usize, refs: I) -> (usize, usize) {
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

fn duplicate_length(data: &[u8], i: usize, j: usize) -> usize {
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
