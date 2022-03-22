use std::env;
use std::error::Error;
use std::ffi::OsString;

/// Gets input file path from cli args
pub fn get_file_path() -> Result<OsString, Box<dyn Error>> {
    // quite simple, but for a real-world production app , i would have used a cli parser such as clap
    match env::args_os().nth(1) {
        Some(f_name) => Ok(f_name),
        None => Err(From::from(
            "missing cli argument! Please specify input csv file path as a first argument.",
        )),
    }
}
