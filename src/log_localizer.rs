use std::fs::{File, self};
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use std::process;
use std::path::PathBuf;

use crate::Config;

pub struct LogLocalizer {
    pub logs_to_localize: Vec<String>,
    pub files: Vec<(PathBuf, File)>,
}

impl Default for LogLocalizer {
    fn default() -> Self {
        Self {
            logs_to_localize: Vec::new(),
            files: Vec::new(),
        }
    }
}

impl LogLocalizer {
    pub fn run(&mut self) -> Result<()> {
        for (file_path, file) in self.files.iter() {
            println!("Reading file: {:?}", file_path);

            let reader = BufReader::new(file);

            for (i, line) in reader.lines().enumerate() {
                let line = line?;
                for log in self.logs_to_localize.iter() {
                    if line.contains(log) {
                        println!("{}: {}", file_path.display(), i+1);
                    }
                }
            }
        }
        println!("");
        
        Ok(())
    }

    pub fn new(config: Config) -> Self {
        let mut log_localizer = LogLocalizer::default();
        log_localizer.get_logs_to_localize(config);
        log_localizer.get_files_to_read("./src");

        log_localizer
    }
        
    pub fn get_logs_to_localize(&mut self, config: Config) {
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
                    Ok(line) => self.logs_to_localize.push(line),
                    Err(error) => {
                        eprintln!("Error reading line: {}", error);
                        process::exit(1);
                    }
                }
            }
        }

        if config.query.len() > 0 {
            for arg in config.query.iter() {
                self.logs_to_localize.push(arg.to_string());
            }
        }
    }

    pub fn get_files_to_read(&mut self, directory_path: &str) {
        self.files = Vec::new();

        let paths = match fs::read_dir(directory_path) {
            Ok(paths) => paths,
            Err(error) => {
                eprintln!("Error reading directory: {}", error);
                process::exit(1);
            }
        };

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
                Ok(file) => self.files.push((file_path, file)),
                Err(error) => {
                    eprintln!("Error opening file: {}", error);
                    process::exit(1);
                }
            };
        } 
    }
}

