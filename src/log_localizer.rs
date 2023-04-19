use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;


pub fn get_logs_to_localize(args: Vec<String>) -> Vec<String> {
    let mut logs_to_localize: Vec<String> = Vec::new();
    
    match args[1].as_str() {
        "--file" => {
            let file_path = &args[2];

            if !Path::new(file_path).exists() {
                eprintln!("File does not exist");
                process::exit(1);
            }

            let file = match File::open(file_path) {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("Error opening file: {}", error);
                    process::exit(1);
                }
            };

            let reader = BufReader::new(file);

            for line_result in reader.lines() {
                match line_result {
                    Ok(line) => logs_to_localize.push(line),
                    Err(error) => {
                        eprintln!("Error reading line: {}", error);
                        process::exit(1);
                    }
                }
            }
        }
        "--text" => {
            for arg in args[2..].iter() {
                logs_to_localize.push(arg.to_string());
            }
        }
        _ => {
            eprintln!("Invalid option. Use --file or --text");
        }
    }

    logs_to_localize
}

