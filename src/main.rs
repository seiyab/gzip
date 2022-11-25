use chrono::Local;
use std::io::{self, Read, Write};

mod deflate;
mod gzip;

fn main() -> io::Result<()> {
    let mut buf = Vec::<u8>::new();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    stdin.read_to_end(&mut buf)?;

    let gzipped = gzip::gzip(&buf, &Local::now());

    stdout.write(&gzipped[..])?;

    Ok(())
}
