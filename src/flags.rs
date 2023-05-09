use std::process;

const VALID_FLAGS: [&str; 8] = ["--query", "--insensitive", "--path", "--number-line", "-q", "-i", "-p", "-n"];

pub struct Flags {
    pub is_case_sensitive: bool,
    pub show_line_numbers: bool,
    pub queries: Vec<String>,
    pub paths: Vec<String>,
}

impl Flags {
    pub fn from_args(args: &[String]) -> Self {
        if args.len() < 3 {
            eprintln!("Not enough arguments");
            process::exit(1);
        }

       let mut flags = Self {
            is_case_sensitive: true,
            show_line_numbers: false,
            queries: Vec::new(),
            paths: Vec::new(),
        };

        for flag in args.iter() {
            match flag.as_str() {
                "--insensitive" | "-i" => {
                    flags.is_case_sensitive = false;
                }
                "--number-line" | "-n" => {
                    flags.show_line_numbers = true;
                }
                "--query" | "-q" => {
                   flags.queries.append(&mut Flags::get_args_by_flag(args, flag)); 
                }
                "--path" | "-p" => {
                    flags.paths.append(&mut Flags::get_args_by_flag(args, flag)); 
                }
                _ => {}
            }
        }

        flags
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
