
use std::fs;
use std::fmt;
use std::result;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SMSRConfiguration{
    pub from: String,
    pub smtp_server: String,
    pub port: u16,
    pub user: String,
    pub passwd: String,
    pub starttls: bool,
}


impl fmt::Display for SMSRConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
           write!(f, "{}, {}, {}, {}, {} \n",self.from, self.smtp_server, self.port, self.user, self.passwd)
    }
}



pub fn parse_configfile(config_file: String) -> result::Result<SMSRConfiguration,String>{
    let s = fs::read_to_string(config_file).map_err(|e| e.to_string())?;
    let c: SMSRConfiguration = serde_json::from_str(&s).map_err(|e| e.to_string())?;
    Ok(c)
}