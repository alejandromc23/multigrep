use std::fs::{File, self};
use std::io::{BufRead, BufReader, Result};
use std::path::{Path, PathBuf};
use std::process;

use crate::flags::Flags;

pub struct Multigrep {
    pub flags: Flags,
}

impl Multigrep {
    pub fn run(&mut self) -> Result<()> {
        let queries = self.get_queries();
        let files = self.get_files();

        println!("\nQueries to localize:");
        for (index, argument) in queries.iter().enumerate() {
            println!("{}: {}", index, argument);
        }
        println!("");

        for (file_path, file) in files.iter() {
            println!("Reading file: {:?}", file_path);

            let reader = BufReader::new(file);

            for (i, line) in reader.lines().enumerate() {
                let line = line?;
                for queries in queries.iter() {
                    if line.contains(queries) {
                        println!("Found at line: {}", i+1);
                    }
                }
            }
            println!("");
        }
        
        Ok(())
    }

    pub fn new(flags: Flags) -> Self {
        Self {
            flags,
        }
    }
        
    pub fn get_queries(&mut self) -> Vec<String> {
        let mut queries = Vec::new();

        for filename in self.flags.filenames.iter() {
            if !Path::new(filename).exists() {
                eprintln!("File does not exist: {}", filename);
                process::exit(1);
            }

            let file = match File::open(filename) {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("Error opening file: {}", error);
                    process::exit(1);
                }
            };

            let reader = BufReader::new(file);

            for line_result in reader.lines() {
                match line_result {
                    Ok(line) => queries.push(line),
                    Err(error) => {
                        eprintln!("Error reading line: {}", error);
                        process::exit(1);
                    }
                }
            }
        }

        for query in self.flags.queries.iter() {
            queries.push(query.to_string());
        }

        queries
    }

    pub fn get_files(&self) -> Vec<(PathBuf, File)> {
        let mut files = Vec::new();

        for read_path in self.flags.paths.iter() {
            let paths = match fs::read_dir(read_path) {
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
                    Ok(file) => files.push((file_path, file)),
                    Err(error) => {
                        eprintln!("Error opening file: {}", error);
                        process::exit(1);
                    }
                };
            } 
        }

        files
    }
}

