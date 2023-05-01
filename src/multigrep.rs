use std::fs::{File, self};
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use std::process;
use std::path::PathBuf;

use crate::flags::Flags;

pub struct Multigrep {
    pub queries_to_localize: Vec<String>,
    pub files: Vec<(PathBuf, File)>,
}

impl Default for Multigrep {
    fn default() -> Self {
        Self {
            queries_to_localize: Vec::new(),
            files: Vec::new(),
        }
    }
}

impl Multigrep {
    pub fn run(&self) -> Result<()> {
        println!("\nQueries to localize:");
        for (index, argument) in self.queries_to_localize.iter().enumerate() {
            println!("{}: {}", index, argument);
        }
        println!("");

        for (file_path, file) in self.files.iter() {
            println!("Reading file: {:?}", file_path);

            let reader = BufReader::new(file);

            for (i, line) in reader.lines().enumerate() {
                let line = line?;
                for query in self.queries_to_localize.iter() {
                    if line.contains(query) {
                        println!("Found at line: {}", i+1);
                    }
                }
            }
        }
        println!("");
        
        Ok(())
    }

    pub fn new(flags: Flags) -> Self {
        let mut multigrep = Multigrep::default();
        multigrep.get_queries_to_localize(flags);
        multigrep.get_files_to_read("./src");

        multigrep
    }
        
    pub fn get_queries_to_localize(&mut self, flags: Flags) {
        if flags.filename != "" {
            if !Path::new(&flags.filename).exists() {
                eprintln!("File does not exist");
                process::exit(1);
            }

            let file = match File::open(&flags.filename) {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("Error opening file: {}", error);
                    process::exit(1);
                }
            };

            let reader = BufReader::new(file);

            for line_result in reader.lines() {
                match line_result {
                    Ok(line) => self.queries_to_localize.push(line),
                    Err(error) => {
                        eprintln!("Error reading line: {}", error);
                        process::exit(1);
                    }
                }
            }
        }

        if flags.query.len() > 0 {
            for arg in flags.query.iter() {
                self.queries_to_localize.push(arg.to_string());
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

