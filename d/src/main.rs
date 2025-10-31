use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = env::var("GMAIL_USER")?;
    let pass = env::var("GMAIL_PASS")?; // 16-char App Password (no spaces)

    let email = Message::builder()
        .from(user.parse()?)                           // or format!("Name <{}>", user).parse()?
        .to("Recipient <jamadahmed08@gmail.com>".parse()?)
        .subject("Test from Rust via Gmail")
        .body(String::from("Hello from Rust!"))?;

    let creds = Credentials::new(user.clone(), pass);

    // Port 587 + STARTTLS
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent"),
        Err(e) => eprintln!("Send failed: {:?}", e),
    }
    Ok(())
}
