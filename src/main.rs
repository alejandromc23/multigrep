use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::PathBuf;

use log_localizer::Config;
use log_localizer::get_logs_to_localize;
use log_localizer::get_files_to_read;

mod log_localizer;


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args);
    
    let logs_to_localize: Vec<String> = get_logs_to_localize(config);
    for (index, argument) in logs_to_localize.iter().enumerate() {
        println!("{}: {}", index, argument);
    }

    let files_to_read: Vec<(PathBuf, File)> = get_files_to_read("./src");    

    for (file_path, file) in files_to_read {
        println!("Reading file: {:?}", file_path);

        let mut reader = BufReader::new(file);
        let mut file_data = String::new();
        reader.read_to_string(&mut file_data)?;

        println!("{}", file_data);
    }

    Ok(())
}
