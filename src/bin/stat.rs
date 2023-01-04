extern crate gzip;

use gzip::deflate::Symbol;

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buf = Vec::<u8>::new();
    let mut stdin = io::stdin();
    stdin.read_to_end(&mut buf)?;

    let symbols = gzip::deflate::symbolize(&buf);
    let mut lens = vec![0; 300];
    for s in symbols.iter() {
        if let &Symbol::Reference {
            length,
            distance: _,
        } = s
        {
            if length < 300 {
                lens[length as usize] += 1;
            }
        }
    }
    for (i, c) in lens.iter().enumerate() {
        println!("{i}\t{c}");
    }

    Ok(())
}
