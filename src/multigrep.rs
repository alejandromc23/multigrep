use std::fs::{File, read_to_string, read_dir};
use std::io::{Result, Read};
use std::path::{Path, PathBuf};
use std::process;
use std::str;
use colored::Colorize;

use crate::flags::Flags;

pub struct Multigrep {
    flags: Flags,
}

impl Multigrep {
    pub fn new(flags: Flags) -> Self {
        Self {
            flags,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let queries = self.get_queries();
        let files = self.get_files();
        
        Self::show_queries(&queries);

        for file_path in files.iter() {
            if !Self::is_valid_utf8(file_path) {
                continue;
            }

            println!("Reading file: {:?}",file_path);
            
            let file = read_to_string(file_path).unwrap();
            
            file.lines().enumerate().for_each(|(i, file_line)| {
                let mut line = file_line.trim().to_string();

                for query in queries.iter() {
                    let mut output = line.clone();
                    
                    if !self.flags.is_case_sensitive {
                        line = line.to_lowercase();
                    }

                    if line.contains(query) {
                        if self.flags.show_line_numbers {
                            output = format!("{}: {}", i+1, output);
                        }

                        output = output.replace(query, &query.red().to_string());
                        println!("{}", output);
                    }
                }
            });
            println!("");
        }
        
        Ok(())
    }

    pub fn get_queries(&mut self) -> Vec<String> {
        let mut queries = Vec::new();

        self.flags.filenames.iter().for_each(|filename| {
            if !Path::new(filename).exists() {
                Self::exit_with_error(format!("File does not exist: {}", filename));
            }

            let file = read_to_string(filename)
                .expect(&format!("Error reading file: {}", filename));

            let lines = file.lines().map(|line| line.to_string());
            queries.extend(lines);
        });

        self.flags.queries.iter().for_each(|query| {
            let mut string_query = query.to_string();

            if self.flags.is_case_sensitive {
                string_query = string_query.to_lowercase();
            }

            queries.push(string_query);
        });

        queries
    }


    pub fn get_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();

        self.flags.paths.iter().for_each(|path_str| {
            let path = PathBuf::from(path_str);
            Self::process_path(&path, &mut files);
        });

        files
    }

    fn process_path(path: &Path, files: &mut Vec<PathBuf>) {
        if !path.exists() {
            Self::exit_with_error(format!("Path does not exist: {}", path.display()));
        }

        if path.is_file() {
            File::open(path).expect(&format!("Error opening file: {}", path.display()));

            files.push(path.to_path_buf());
            return;
        }

        let paths = read_dir(path).expect(&format!("Error reading path: {}", path.display()));

        for entry in paths {
            match entry {
                Ok(dir_entry) => {
                    Self::process_path(&dir_entry.path(), files);
                },
                Err(error) => {
                    Self::exit_with_error(format!("Error reading path: {}", error));
                }
            }
        }
    }

    fn is_valid_utf8(path: &Path) -> bool {
        let mut file = File::open(path).unwrap();
        let mut bytes = Vec::new();

        file.read_to_end(&mut bytes).unwrap();

        str::from_utf8(&bytes).is_ok()
    }

    fn exit_with_error(error: String) {
        eprintln!("{}", error);
        process::exit(1);
    }

    fn show_queries(queries: &Vec<String>) {
        println!("Queries to localize:");
        queries.iter().enumerate().for_each(|(index, query)| {
            println!("{}: {}", index, query);
        });
        println!("");
    }
}
