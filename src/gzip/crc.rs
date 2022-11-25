pub fn crc(data: &Vec<u8>) -> [u8; 4] {
    let t = table();
    let mut c: usize = 0xff_ff_ff_ff;
    for &d in data.iter() {
        c = t[(c ^ d as usize) & 0xff] ^ (c >> 8);
    }
    c = c ^ 0xff_ff_ff_ff;

    let mut cs: [u8; 4] = [0; 4];
    for (i, &b) in c.to_le_bytes().iter().take(4).enumerate() {
        cs[i] = b
    }
    cs
}

pub fn table() -> &'static [usize; 256] {
    static mut T: [usize; 256] = [0; 256];
    unsafe {
        if T[0] > 0 {
            return &T;
        };
    }
    for n in 0..256 as usize {
        let mut c = n;
        for _ in 0..8 {
            if c & 1 > 0 {
                c = 0xedb88320 ^ (c >> 1);
            } else {
                c = c >> 1;
            }
        }
        unsafe {
            T[n] = c;
        }
    }
    unsafe {
        return &T;
    }
}
