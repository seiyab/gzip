use std::env;

pub struct Args {
    pub filepath: Option<String>,
}

impl Args {
    pub fn parse() -> Result<Self, String> {
        let mut iargs = env::args();
        iargs.next();
        let filepath = iargs.next();
        if let Some(_) = iargs.next() {
            let full_args: String = env::args().reduce(|a, b| a + ", " + &b).unwrap();
            let message = format!("too many arguments: '{full_args}'");
            return Err(message);
        }
        return Ok(Self { filepath });
    }
}
