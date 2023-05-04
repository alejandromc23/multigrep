use std::fs::{File, self};
use std::io::{BufRead, BufReader, Result, Read};
use std::path::{Path, PathBuf};
use std::process;
use std::str;

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
        
        println!("\nQueries to localize:");
        for (index, argument) in queries.iter().enumerate() {
            println!("{}: {}", index, argument);
        }
        println!("");

        for (file_path, file) in files.iter() {
            if !Self::is_valid_utf8(file_path) {
                continue;
            }

            println!("Reading file: {:?}", file_path);

            let reader = BufReader::new(file);

            for (i, line) in reader.lines().enumerate() {
                let mut line = line?;
                for query in queries.iter() {
                    if !self.flags.is_case_sensitive {
                        line = line.to_lowercase();
                    }

                    if line.contains(query) {
                        if self.flags.show_line_numbers {
                            print!("{}: {}", i+1, line);
                            continue;
                        }

                        println!("{}", line);
                    }
                }
            }
            println!("");
        }
        
        Ok(())
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
            let mut string_query = query.to_string();

            if self.flags.is_case_sensitive {
                string_query = string_query.to_lowercase();
            }

            queries.push(string_query);
        }

        queries
    }

    pub fn get_files(&self) -> Vec<(PathBuf, File)> {
        let mut files = Vec::new();

        for read_path in self.flags.paths.iter() {
            let path = PathBuf::from(read_path);
            Self::process_path(&path, &mut files);
        }

        files
    }

    fn process_path(path: &Path, files: &mut Vec<(PathBuf, File)>) {
        if !path.exists() {
            eprintln!("Path does not exist: {}", path.display());
            process::exit(1);
        }

        if path.is_file() {
            let file_result = File::open(path);

            match file_result {
                Ok(file) => files.push((path.to_path_buf(), file)),
                Err(error) => {
                    eprintln!("Error opening file: {}", error);
                    process::exit(1);
                }
            };

            return;
        }

        match fs::read_dir(path) {
            Ok(paths) => {
                for entry in paths {
                    match entry {
                        Ok(dir_entry) => {
                            Self::process_path(&dir_entry.path(), files);
                        },
                        Err(error) => {
                            eprintln!("Error reading path: {}", error);
                            process::exit(1);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading directory: {}", error);
                process::exit(1);
            }
        }
    }

    fn is_valid_utf8(path: &Path) -> bool {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Error opening file: {}", error);
                return false;
            }
        };

        let mut bytes = Vec::new();
        if let Err(error) = file.read_to_end(&mut bytes) {
            eprintln!("Error reading file: {}", error);
            return false;
        }

        str::from_utf8(&bytes).is_ok()
    }
}

