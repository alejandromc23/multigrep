use std::process;
use std::env;

const VALID_FLAGS: [&str; 10] = ["--query", "--insensitive", "--path", "--number-line", "--regexp", "-q", "-i", "-p", "-n", "-e"];

pub struct Flags {
    pub is_case_sensitive: bool,
    pub show_line_numbers: bool,
    pub queries: Vec<String>,
    pub paths: Vec<String>,
    pub regexps: Vec<String>,
}

impl Flags {
    pub fn from_args(args: &[String]) -> Self {
        let mut paths = Vec::new();
        let mut queries = Vec::new();
        let mut regexps = Vec::new();
        let mut is_case_sensitive = true;
        let mut show_line_numbers = false;

        let mut has_query_or_regexp_arg = false;

        args.iter().for_each(|flag| {
            match flag.as_str() {
                "--insensitive" | "-i" => {
                    is_case_sensitive = false;
                }
                "--number-line" | "-n" => {
                    show_line_numbers = true;
                }
                "--query" | "-q" => {
                    has_query_or_regexp_arg = true;
                    queries.append(&mut Self::get_args_by_flag(args, flag));
                }
                "--path" | "-p" => {
                    paths.append(&mut Self::get_args_by_flag(args, flag));
                }
                "--regexp" | "-e" => {
                    has_query_or_regexp_arg = true;
                    regexps.append(&mut Self::get_args_by_flag(args, flag));
                }
                _ => {}
            }
        });

        // If no path argument was passed, use the current directory
        if paths.is_empty() {
            paths.push(env::current_dir().unwrap().to_string_lossy().to_string());
        }

        if !has_query_or_regexp_arg {
            eprintln!("Error: Must specify at least one query or regexp argument.");
            process::exit(1);
        }

        Self {
            is_case_sensitive,
            show_line_numbers,
            queries,
            paths,
            regexps,
        }
    }

    pub fn get_args_by_flag(args: &[String], flag: &str) -> Vec<String> {
        let mut args_by_flag = vec![];
        let mut iter = args.iter();

        while let Some(arg) = iter.next() {
            if arg == flag {
                for arg in iter {
                    if VALID_FLAGS.contains(&arg.as_str()) {
                        break;
                    }
                    args_by_flag.push(arg.clone());
                }
                break;
            }
        }

        args_by_flag
    }
}
