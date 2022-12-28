use crate::deflate::code_length_symbol::CodeLengthSymbol;

pub fn symbolize_code_length<'a, I: Iterator<Item = &'a u8>>(it: I) -> Vec<CodeLengthSymbol> {
    let mut ret: Vec<CodeLengthSymbol> = Vec::new();
    for &code_length in it {
        if code_length == 0 {
            if let Some(l2) = last2(&ret) {
                if let [_, CodeLengthSymbol::RepeatZero(z)] = l2 {
                    if z < 138 {
                        ret.pop();
                        ret.push(CodeLengthSymbol::RepeatZero(z + 1));
                        continue;
                    }
                }
                if let [CodeLengthSymbol::Literal(0), CodeLengthSymbol::Literal(0)] = l2 {
                    ret.pop();
                    ret.pop();
                    ret.push(CodeLengthSymbol::RepeatZero(3));
                    continue;
                }
            }
        }
        if let Some(l3) = last3(&ret) {
            if let [CodeLengthSymbol::Literal(x), CodeLengthSymbol::Literal(y), CodeLengthSymbol::Literal(z)] =
                l3
            {
                if x == code_length && y == code_length && z == code_length {
                    ret.pop();
                    ret.pop();
                    ret.push(CodeLengthSymbol::CopyPrevious(3));
                    continue;
                }
            }
            if let [_, CodeLengthSymbol::Literal(x), CodeLengthSymbol::CopyPrevious(y)] = l3 {
                if x == code_length && y < 6 {
                    ret.pop();
                    ret.push(CodeLengthSymbol::CopyPrevious(y + 1));
                    continue;
                }
            }
        }
        ret.push(CodeLengthSymbol::Literal(code_length));
    }
    return ret;
}

fn last2<T: Clone>(v: &Vec<T>) -> Option<[T; 2]> {
    let l = v.len();
    if l < 2 {
        return None;
    }
    return Some([v[l - 2].clone(), v[l - 1].clone()]);
}

fn last3<T: Clone>(v: &Vec<T>) -> Option<[T; 3]> {
    let l = v.len();
    if l < 3 {
        return None;
    }
    return Some([v[l - 3].clone(), v[l - 2].clone(), v[l - 1].clone()]);
}
