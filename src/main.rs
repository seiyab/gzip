use chrono::Local;
use std::io::{self, Read, Write};

mod args;
mod deflate;
mod gzip;

fn main() -> io::Result<()> {
    let mut buf = Vec::<u8>::new();
    let a = args::Args::parse().unwrap();
    let (mut input, mut output) = if let Some(_) = a.filepath {
        todo!();
    } else {
        (io::stdin(), io::stdout())
    };
    input.read_to_end(&mut buf)?;

    let gzipped = gzip::gzip(&buf, &Local::now());

    output.write(&gzipped[..])?;

    Ok(())
}
