use std::env;
use std::io::Result;
use log_localizer::get_logs_to_localize;

mod log_localizer;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let logs_to_localize: Vec<String> = get_logs_to_localize(args);

    for (index, argument) in logs_to_localize.iter().enumerate() {
        println!("{}: {}", index, argument);
    }

    Ok(())
}
