use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

pub mod types {
    use super::*;

    pub trait Sendable {
        fn send(&self) -> Result<(), SendError>;
    }

    pub struct Email {
        pub to: String,
        pub from: String,
    }

    pub enum SendError {
        InvalidSender(String),
        InvalidReceiver(String),
        TimedOutError(String),
        SendFailure(String),
    }

    impl Sendable for Email {
        fn send(&self) -> Result<(), SendError> {
            if self.from.is_empty() {
                return Err(SendError::InvalidSender("Sender missing".into()));
            }
            if self.to.is_empty() {
                return Err(SendError::InvalidReceiver("Receiver missing".into()));
            }

            let body_message = "Hey honey, I would like you to know that you're such a beautiful princess. What are you going to do tomorrow?";

            let email = Message::builder()
                .from(self.from.parse().unwrap())
                .to(self.to.parse().unwrap())
                .subject("Hey mommy I'm learning Rust.")
                .body(body_message.to_string())
                .unwrap();

            let creds = Credentials::new("Mailtrap_user".into(), "Mailtrap_pass".into());
            let mailer = SmtpTransport::relay("smtp.mailtrap.io")
                .map_err(|e| SendError::SendFailure(format!("Relay error: {e:?}")))?
                .credentials(creds)
                .build();

            mailer
                .send(&email)
                .map_err(|e| SendError::SendFailure(format!("Send error: {e:?}")))?;

            Ok(())
        }
    }
}
// 8a60ae18d2e8e839526667d77421d8cf