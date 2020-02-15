extern crate lettre;

use lettre::{SmtpClient, Transport};
use lettre::SmtpTransport;
use lettre::SendableEmail;
use lettre::smtp::authentication::{Mechanism, Credentials};
use lettre_email::EmailBuilder;


fn main() {

    let email: SendableEmail = EmailBuilder::new()
        .to("someone@gmail.com")
        .from("myself@gmail.com")
        .subject("some subject")
        .text("some text")
        .build()
        .unwrap()
        .into();

    let creds = Credentials::new(
        "login".to_string(),
        "app_password".to_string(),
    );

    let mut mailer: SmtpTransport = SmtpClient::new_simple("smtp.gmail.com").unwrap().credentials(creds).transport();
    let result = mailer.send(email);

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());
}