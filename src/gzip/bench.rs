#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{self, Write},
        path::Path,
        time::SystemTime,
    };

    use chrono::DateTime;

    use crate::gzip::{gzip, Config};

    #[test]
    #[ignore]
    fn bench_oas() {
        bench("oas (188K)", || bench_file("testdata/oai-spec-3.1.0.md"))
    }

    #[test]
    #[ignore]
    fn bench_js() {
        bench("vendor.js (916K)", || bench_file("testdata/vendor.js"))
    }

    fn bench_file<P: AsRef<Path>>(filepath: P) -> usize {
        let input = File::open(
            Path::new(file!())
                .parent()
                .expect("failed to find parent")
                .join(&filepath),
        )
        .expect("failed to open file");
        let mut output = Counter(0);
        gzip(&mut output, input, cfg(1_000_000));
        output.0
    }

    fn cfg(buf_size: usize) -> Config {
        Config {
            mtime: DateTime::default(),
            buf_size,
        }
    }

    fn bench<F: Fn() -> usize>(name: &str, run: F) {
        let mut sum = 0f64;
        let mut size = 0usize;
        for _ in 0..5 {
            let start = SystemTime::now();
            size = run();
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
        let size_str = if size > 1_000_000_000 {
            format!("{:.2}[GiB]", (size) as f64 / 2f64.powi(30))
        } else if size > 1_000_000 {
            format!("{:.2}[MiB]", size as f64 / 2f64.powi(20))
        } else if size > 1_000 {
            format!("{:.2}[KiB]", size as f64 / 2f64.powi(10))
        } else {
            format!("{size}[B]")
        };
        println!("{name}: {avg}[ms], {size_str}");
    }

    struct Counter(usize);

    impl Write for Counter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0 = self.0 + buf.len();
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
}
