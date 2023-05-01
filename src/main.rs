use std::env;
use std::io::Result;

use log_localizer::LogLocalizer;
use flags::Flags;

mod log_localizer;
mod flags;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let flags: Flags = Flags::from_args(&args);

    let log_localizer: LogLocalizer = LogLocalizer::new(flags);

    log_localizer.run()
}
