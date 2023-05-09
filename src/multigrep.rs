use std::fs::{File, read_to_string, read_dir};
use std::io::{Result, Read};
use std::path::{Path, PathBuf};
use std::process;
use std::str;
use rand::Rng;
use regex::Regex;

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
        let regexps = self.get_regexps();
        let files = self.get_files();

        Self::show_inputs_to_search(&queries, &regexps);
        self.show_coincidences(queries, regexps, files);

        Ok(())
    }

    fn get_queries(&mut self) -> Vec<(String, String)> {
        self.flags.queries.iter().map(|query| {
            let mut string_query = query.to_string();

            if self.flags.is_case_sensitive {
                string_query = string_query.to_lowercase();
            }

            (string_query.clone(), Self::get_query_replacement(&string_query))
        }).collect()
    }

    fn get_regexps(&mut self) -> Vec<(Regex, String)> {
        self.flags.regexps.iter().map(|regexp_str| {
            let regex = Regex::new(regexp_str)
                .expect(&format!("Error parsing regexp: {}", regexp_str));
            let color_replacement = Self::get_random_color();

            (regex, color_replacement)
        }).collect()
    }

    fn get_files(&self) -> Vec<PathBuf> {
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

        for entry_result in paths {
            let entry = entry_result.expect(&format!("Error reading path: {}", path.display()));
            Self::process_path(&entry.path(), files);
        }
    }

    fn show_coincidences(&self, queries: Vec<(String, String)>, regexps: Vec<(Regex, String)>, files: Vec<PathBuf>) {
        for file_path in files.iter() {
            if !Self::is_valid_utf8(file_path) {
                continue;
            }

            println!("{}Reading file: {:?}{}", "\x1b[38;2;255;165;0m", file_path, "\x1b[0m");

            let file = read_to_string(file_path).unwrap();

            file.lines()
                .enumerate()
                .for_each(|(i, file_line)| self.show_line_coincidences(file_line, i as u32 + 1, &queries, &regexps));

            println!("");
        }
    }

    fn show_line_coincidences(&self, line: &str, line_number: u32, queries: &Vec<(String, String)>, regexps: &Vec<(Regex, String)>) {
        let mut line = line.trim().to_string();
        let mut has_line_matches = false;

        queries.iter().for_each(|(query, replacement)| {
            if !self.flags.is_case_sensitive {
                line = line.to_lowercase();
            }

            if line.contains(query) {
                line = line.replace(query, replacement);
                has_line_matches = true;
            }
        });

        regexps.iter().for_each(|(regexp, color_replacement)| {
            if regexp.is_match(&line) {
                line = regexp
                    .replace_all(&line,  format!("{}$0{}", color_replacement, "\x1b[0m"))
                    .to_string();
                has_line_matches = true;
            }
        });

        if !has_line_matches {
            return;
        }

        if self.flags.show_line_numbers {
            line = format!("{}: {}", line_number, line);
        }

        println!("{}", line);
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

    fn show_inputs_to_search(queries: &Vec<(String, String)>, regexps: &Vec<(Regex, String)>) {
        println!("Queries to localize:");
        queries.iter().enumerate().for_each(|(index, query)| {
            println!("{}: {}", index, query.1);
        });
        regexps.iter().enumerate().for_each(|(index, regexp)| {
            println!("{}: {}", index + queries.len(), Self::get_regex_replacement(&regexp.0, regexp.1.clone())); 
        });
        println!("");
    }

    fn get_query_replacement(pattern: &str) -> String {
        let color_code = Self::get_random_color();
        format!("{}{}{}", color_code, pattern, "\x1b[0m")
    }

    fn get_regex_replacement(regexp: &Regex, color_replacement: String) -> String {
        format!("{}{}{}", color_replacement, regexp.as_str(), "\x1b[0m")
    }

    fn get_random_color() -> String {
        let mut rng = rand::thread_rng();
        format!("\x1b[38;5;{}m", rng.gen_range(16..=232))
    }
}
