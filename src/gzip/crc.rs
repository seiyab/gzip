pub struct Crc {
    value: u32,
}

impl Crc {
    pub fn new() -> Self {
        Self {
            value: 0xff_ff_ff_ff,
        }
    }

    pub fn append(self, data: &[u8]) -> Self {
        let mut c = self.value;
        for &d in data.iter() {
            c = TABLE[((c ^ u32::from(d)) & 0xff) as usize] ^ (c >> 8);
        }
        Self { value: c }
    }

    pub fn get(&self) -> [u8; 4] {
        (self.value ^ 0xff_ff_ff_ff).to_le_bytes()
    }
}

const TABLE: [u32; 256] = make_table();

const fn make_table() -> [u32; 256] {
    let mut t: [u32; 256] = [0; 256];
    let mut n: usize = 0;
    while n < 256 {
        t[n] = table_elem(n);
        n += 1;
    }
    return t;
}

const fn table_elem(n: usize) -> u32 {
    let mut c = n as u32;
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
