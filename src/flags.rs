use std::process;

pub struct Flags {
    pub query: Vec<String>,
    pub filename: String,
}

impl Flags {
    pub fn from_args(args: &[String]) -> Self {
        if args.len() < 3 {
            eprintln!("Not enough arguments");
            process::exit(1);
        }

        let mut flags = Self {
            query: Vec::new(),
            filename: String::from(""),
        };
        
        match args[1].as_str() {
            "--file" => {
                flags.filename = args[2].clone();
            }
            "--query" => {
                for arg in args[2..].iter() {
                    flags.query.push(arg.to_string());
                }
            }
            _ => {
                eprintln!("Invalid option. Use --file or --text");
                process::exit(1);
            }
        }

        flags
    }
}
