mod config;
mod parameters;
mod email;

use std::result;

use crate::config::*;
use crate::parameters::*;
use crate::email::*;

fn main() -> result::Result<(),String> {
    let config_file = "/etc/smsr.conf";
    send_email(parse_configfile(config_file.to_string())?, parse_cmd_parameters())    
}
