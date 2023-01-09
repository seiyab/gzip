use chrono::Local;
use std::{fs::File, io};

mod args;
mod deflate;
mod gzip;

fn main() -> io::Result<()> {
    let a = args::Args::parse().unwrap();
    if let Some(filepath) = a.filepath {
        gzip::gzip(
            File::create(filepath.clone() + ".gz")?,
            &Local::now(),
            File::open(&filepath)?,
        )
    } else {
        gzip::gzip(io::stdout(), &Local::now(), io::stdin())
    };

    Ok(())
}
