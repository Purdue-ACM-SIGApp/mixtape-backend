use anyhow::Result;
use async_trait::async_trait;
use phonenumber::country::Id;
use twilio::{Client, OutboundMessage};

pub fn clean_phone_number(dirty: &str) -> Result<String> {
    let number = phonenumber::parse(Some(Id::US), dirty)?;

    Ok(number.format().mode(phonenumber::Mode::E164).to_string())
}

#[async_trait]
pub trait PhoneClient {
    async fn send(&self, phone_number: &str, body: &str) -> Result<()>;
}

#[allow(dead_code)]
pub struct SentMessage {
    destination: String,
    body: String,
}

#[derive(Default)]
pub struct MockPhoneClient {
    pub sent_messages: async_mutex::Mutex<Vec<SentMessage>>,
}

#[async_trait]
impl PhoneClient for MockPhoneClient {
    async fn send(&self, phone_number: &str, body: &str) -> Result<()> {
        self.sent_messages.lock().await.push(SentMessage {
            destination: phone_number.to_string(),
            body: body.to_string(),
        });

        Ok(())
    }
}

pub struct TwilioPhoneClient {
    client: Client,
}

#[async_trait]
impl PhoneClient for TwilioPhoneClient {
    async fn send(&self, phone_number: &str, body: &str) -> Result<()> {
        // Override phone number if fake and on dev
        let phone_number = if crate::IS_DEV && phone_number.contains("000338") {
            dotenv!("FAKE_NUMBER")
        } else {
            phone_number
        };

        self.client
            .send_message(OutboundMessage::new(dotenv!("TWILIO_NUMBER"), phone_number, body))
            .await?;
        Ok(())
    }
}

impl TwilioPhoneClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(dotenv!("TWILIO_ACCOUNT_ID"), dotenv!("TWILIO_AUTH_TOKEN")),
        }
    }
}

impl Default for TwilioPhoneClient {
    fn default() -> Self {
        Self::new()
    }
}