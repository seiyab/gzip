use self::gzip::Config;
use chrono::Local;
use std::{fs::File, io};

mod args;
mod deflate;
mod gzip;

const BUF_SIZE: usize = 1_000_000;

fn main() -> io::Result<()> {
    let a = args::Args::parse().unwrap();
    if let Some(filepath) = a.filepath {
        gzip::gzip(
            File::create(filepath.clone() + ".gz")?,
            File::open(&filepath)?,
            Config {
                mtime: Local::now(),
                buf_size: BUF_SIZE,
            },
        )
    } else {
        gzip::gzip(
            io::stdout(),
            io::stdin(),
            Config {
                mtime: Local::now(),
                buf_size: BUF_SIZE,
            },
        )
    };

    Ok(())
}
