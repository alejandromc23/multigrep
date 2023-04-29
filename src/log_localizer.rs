use std::fs::{File, self};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;
use std::path::PathBuf;

pub struct Config {
    query: Vec<String>,
    filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Config {
        let mut config = Config {
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

pub fn get_logs_to_localize(config: Config) -> Vec<String> {
    let mut logs_to_localize: Vec<String> = Vec::new();
 

    if config.filename != "" {
        if !Path::new(&config.filename).exists() {
            eprintln!("File does not exist");
            process::exit(1);
        }

        let file = match File::open(&config.filename) {
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

    if config.query.len() > 0 {
        for arg in config.query.iter() {
            logs_to_localize.push(arg.to_string());
        }
    }

    logs_to_localize
}


pub fn get_files_to_read(directory_path: &str) -> Vec<(PathBuf, File)> {
    let paths = match fs::read_dir(directory_path) {
        Ok(paths) => paths,
        Err(error) => {
            eprintln!("Error reading directory: {}", error);
            process::exit(1);
        }
    };

    let mut files = Vec::new();

    for path in paths {
        let path = match path {
            Ok(dir_entry) => dir_entry,
            Err(error) => {
                eprintln!("Error reading path: {}", error);
                process::exit(1);
            }
        };

        let file_path = path.path();
        let file_result = File::open(&file_path);

        match file_result {
            Ok(file) => files.push((file_path, file)),
            Err(error) => {
                eprintln!("Error opening file: {}", error);
                process::exit(1);
            }
        };
    } 

   files 
}
