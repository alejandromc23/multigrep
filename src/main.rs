use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, self};
use std::path::Path;
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut input_args: Vec<String> = Vec::new();

    if args.len() < 3 { 
        eprintln!("Usage:");
        eprintln!("  log-localizer --file path-to-file");
        eprintln!("  log-localizer --text \"log text\"");
        process::exit(1);
    }

    match args[1].as_str() {
        "--file" => {
            let file_path = &args[2];

            if !Path::new(file_path).exists() {
                eprintln!("File does not exist");
                process::exit(1);
            }

            let file = File::open(file_path)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                input_args.push(line?);
            }
        }
        "--text" => {
            for arg in args[2..].iter() {
                input_args.push(arg.to_string());
            }
        }
        _ => {
            eprintln!("Invalid option. Use --file or --text");
        }
    }

    for (index, argument) in input_args.iter().enumerate() {
        println!("{}: {}", index, argument);
    }

    Ok(())
}
