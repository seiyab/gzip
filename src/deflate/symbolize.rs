use super::{locator::Locator, symbol::Symbol};

pub fn symbolize(data: &[u8]) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut locator = Locator::new();
    let mut cursor = 0usize;
    for i in 0..data.len() {
        if cursor > i {
            continue;
        }
        let triple = triple_at(data, i);
        let new_symbol = {
            let locs = triple.and_then(|t| locator.locate(&t));
            match locs {
                None => Symbol::Literal(data[i]),
                Some(locs) => {
                    let (length, distance) =
                        longest_duplicate(data, i, locs.into_iter().rev().take(10).copied());
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
        if let Some(t) = &triple {
            locator.register(t, i);
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

fn triple_at(data: &[u8], i: usize) -> Option<[u8; 3]> {
    if data.len() <= i + 2 {
        None
    } else {
        Some([data[i], data[i + 1], data[i + 2]])
    }
}
