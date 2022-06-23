

use crate::config::*;
use crate::parameters::*;

use lettre::address::AddressError;
use lettre::transport::smtp::SmtpTransportBuilder;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport, transport::smtp::Error};

pub fn send_email(config: SMSRConfiguration, args: SMSRParameters) -> Result<(),String> {
    let email = Message::builder()
        .from(config.from.parse().map_err(|e: AddressError| e.to_string())?)
        .reply_to(config.from.parse().map_err(|e: AddressError| e.to_string())?)
        .to(args.to.parse().map_err(|e:AddressError| e.to_string())?)
        .subject(args.subject)
        .body(String::from(args.body))
        .map_err(|e: lettre::error::Error| e.to_string())?;

    let creds = Credentials::new(config.user, config.passwd);

    let smtp_builder: Result<SmtpTransportBuilder, Error>;

    if config.starttls{
        smtp_builder = SmtpTransport::starttls_relay(&config.smtp_server);
    }else{
        smtp_builder = SmtpTransport::relay(&config.smtp_server);
    }

    let mailer = smtp_builder
        .map_err(|e| e.to_string())?
        .port(config.port)
        .credentials(creds)
        .build();

    match mailer.send(&email){
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}
