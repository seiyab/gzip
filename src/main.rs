use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buf: [u8; 10] = [0; 10];
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        let n = stdin.read(&mut buf)?;
        if n == 0 {
            break;
        };
        stdout.write(&buf[0..n])?;
    }

    Ok(())
}
