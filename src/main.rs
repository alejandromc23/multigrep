use std::env;
use std::io::Result;
use std::process;

use log_localizer::LogLocalizer;

mod log_localizer;

pub struct Config {
    query: Vec<String>,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Self {
        if args.len() < 3 {
            eprintln!("Not enough arguments");
            process::exit(1);
        }

        let mut config = Self {
            query: Vec::new(),
            filename: String::from(""),
        };
        
        match args[1].as_str() {
            "--file" => {
                config.filename = args[2].clone();
            }
            "--query" => {
                for arg in args[2..].iter() {
                    config.query.push(arg.to_string());
                }
            }
            _ => {
                eprintln!("Invalid option. Use --file or --text");
                process::exit(1);
            }
        }

        config
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args);

    let mut log_localizer: LogLocalizer = LogLocalizer::new(config);

    println!("\nLogs to localize:");
    for (index, argument) in log_localizer.logs_to_localize.iter().enumerate() {
        println!("{}: {}", index, argument);
    }
    println!("");

    log_localizer.run()
}
