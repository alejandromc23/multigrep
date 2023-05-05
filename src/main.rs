use std::env;
use std::io::Result;
use std::process;

use flags::Flags;
use multigrep::Multigrep;

mod multigrep;
mod flags;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let flags: Flags = Flags::from_args(&args);

    let mut multigrep: Multigrep = Multigrep::new(flags);

    match multigrep.run() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        },
    }
}
