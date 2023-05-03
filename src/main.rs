use std::env;
use std::io::Result;

use flags::Flags;
use multigrep::Multigrep;

mod multigrep;
mod flags;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let flags: Flags = Flags::from_args(&args);

    let mut multigrep: Multigrep = Multigrep::new(flags);

    multigrep.run()
}
