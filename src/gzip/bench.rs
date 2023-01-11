#[cfg(test)]
mod tests {
    use std::{fs::File, io, path::Path, time::SystemTime};

    use chrono::DateTime;

    use crate::gzip::{gzip, Config};

    #[test]
    #[ignore]
    fn bench_oas() {
        bench("oas (188K)", || {
            bench_file("testdata/oai-spec-3.1.0.md");
        })
    }

    fn bench_file<P: AsRef<Path>>(filepath: P) {
        let input = File::open(
            Path::new(file!())
                .parent()
                .expect("failed to find parent")
                .join(&filepath),
        )
        .expect("failed to open file");
        let output = io::sink();
        gzip(output, input, cfg(1_000_000));
    }

    fn cfg(buf_size: usize) -> Config {
        Config {
            mtime: DateTime::default(),
            buf_size,
        }
    }

    fn bench<F: Fn() -> ()>(name: &str, run: F) {
        let mut sum = 0f64;
        for _ in 0..5 {
            let start = SystemTime::now();
            run();
            let end = SystemTime::now();
            let ms: u32 = end
                .duration_since(start)
                .expect("Clock may have gone backwards")
                .as_millis()
                .try_into()
                .expect("failed to convert to u32");
            sum += f64::from(ms);
        }
        let avg = sum / 5f64;
        println!("{name}: {avg}[ms]")
    }
}
