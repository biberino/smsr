use std::fmt;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct SMSRParameters{
    #[clap(short, long, value_parser)]
    body: String,
    #[clap(short, long, value_parser, default_value = "Email from SMSR")]
    subject: String,
    #[clap(short, long, value_parser)]
    to: String, // Comma seperated String of recipients
}

impl fmt::Display for SMSRParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
           write!(f, "{}, {}, {} \n",self.to, self.subject, self.body)
    }
}

pub fn parse_cmd_parameters() -> SMSRParameters{
    SMSRParameters::parse()
}