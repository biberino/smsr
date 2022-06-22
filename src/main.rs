mod config;
mod parameters;

use std::result;

use crate::config::*;
use crate::parameters::*;

fn main() -> result::Result<(),String> {
    let config_file = "/home/biber/robin/projekte/smsr/smsr.conf";
    let config = parse_configfile(config_file.to_string())?;

    let args = parse_cmd_parameters();
    print!("{}", args);

    
    Ok(())
}
