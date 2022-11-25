pub fn crc(data: &Vec<u8>) -> [u8; 4] {
    let mut c: usize = 0xff_ff_ff_ff;
    for &d in data.iter() {
        c = TABLE[(c ^ d as usize) & 0xff] ^ (c >> 8);
    }
    c = c ^ 0xff_ff_ff_ff;

    let mut cs: [u8; 4] = [0; 4];
    for (i, &b) in c.to_le_bytes().iter().take(4).enumerate() {
        cs[i] = b
    }
    cs
}

const TABLE: [usize; 256] = make_table();

const fn make_table() -> [usize; 256] {
    let mut t: [usize; 256] = [0; 256];
    let mut n: usize = 0;
    while n < 256 {
        t[n] = table_elem(n);
        n += 1;
    }
    return t;
}

const fn table_elem(n: usize) -> usize {
    let mut c = n;
    let mut i: usize = 0;
    while i < 8 {
        if c & 1 > 0 {
            c = 0xedb88320 ^ (c >> 1);
        } else {
            c = c >> 1;
        }
        i = i + 1;
    }
    return c;
}
