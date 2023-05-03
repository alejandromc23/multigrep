use std::process;

const VALID_FLAGS: [&str; 8] = ["--file", "--query", "--insensitive", "--path", "-f", "-q", "-i", "-p"];

pub struct Flags {
    pub is_case_sensitive: bool,
    pub queries: Vec<String>,
    pub filenames: Vec<String>,
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
            queries: Vec::new(),
            filenames: Vec::new(),
            paths: Vec::new(),
        };

        for flag in args.iter() {
            match flag.as_str() {
                "--file" | "-f" => {
                    flags.filenames.append(&mut Flags::get_args_by_flag(args, flag));
                }
                "--query" | "-q" => {
                   flags.queries.append(&mut Flags::get_args_by_flag(args, flag)); 
                }
                "--insensitive" | "-i" => {
                    flags.is_case_sensitive = false;
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
