use chrono::Local;
use std::{
    fs::File,
    io::{self, Read, Write},
};

mod args;
mod deflate;
mod gzip;

fn main() -> io::Result<()> {
    let mut buf = Vec::<u8>::new();
    let a = args::Args::parse().unwrap();
    let (mut input, mut output): (Box<dyn Read>, Box<dyn Write>) =
        if let Some(filepath) = a.filepath {
            (
                Box::new(File::open(&filepath)?),
                Box::new(File::create(filepath + ".gz")?),
            )
        } else {
            (Box::new(io::stdin()), Box::new(io::stdout()))
        };
    input.read_to_end(&mut buf)?;

    let gzipped = gzip::gzip(&buf, &Local::now());

    output.write(&gzipped[..])?;

    Ok(())
}
