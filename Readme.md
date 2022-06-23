## Simple Mail Sender Rust (smsr)
### Project Desciption
 This is a straitforward and easy to configure program to send emails from the commandline, shellscripts, etc ...

 ### Installation

 Grab the latest binary from the release page and make it executable.

 ### Configuration

 Create a config file named **/etc/smsr.conf** with the following structure

 ```json
{
    "from": "example@abc.org", //email address from which the email will be send
    "smtp_server": "mail.abc.org", // SMTP server address of your email provider
    "port": 587, // SMTP port of your email provider
    "user": "example@abc.org", //username to login into your email provider
    "passwd": "superstrongpasswd", //password to login into your email provider
    "starttls": true // true will use STARTTLS, false will use classic TLS
}
 ```

 ### Usage
 To send an email from the commandline use:
 ```console
$ ./smsr --subject "Email from SMSR" --body "Your backups are up to date :)" --to "recipient@whatever.org"
 ```

 