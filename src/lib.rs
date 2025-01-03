use std::sync::Arc;
use crate::payment::Payment;
use crate::token::Token;

mod payment;
mod token;

pub struct ClientOptionsBuilder {
    pub merchant_key: String,
    pub dev_mode: bool
}

impl Default for ClientOptionsBuilder {
    fn default() -> Self {
        ClientOptionsBuilder {
            merchant_key: "".to_string(),
            dev_mode: false
        }
    }
}

impl ClientOptionsBuilder {
    pub fn with_merchant_key(merchant_key: String) -> Self {
        ClientOptionsBuilder {
            merchant_key,
            dev_mode: false
        }
    }

    pub fn enable_dev_mode(mut self) -> Self {
        self.dev_mode = true;
        self
    }

    pub fn build(&self) -> ClientOptions {
        ClientOptions {
            merchant_key: Some(self.merchant_key.clone()),
            api_base: if self.dev_mode {
                "https://test.vepara.com.tr/ccpayment".to_string()
            } else {
                "https://app.vepara.com.tr/ccpayment".to_string()
            },
            reqwest_client: reqwest::Client::new()
        }
    }
}

pub struct ClientOptions {
    pub merchant_key: Option<String>,
    pub api_base: String,
    pub reqwest_client: reqwest::Client,
}

#[derive(Clone)]
pub struct Client {
    pub payment: Payment,
    pub token: Token
}

impl Client {
    pub fn new(options: ClientOptions) -> Self {
        let options = Arc::new(options);
        Client {
            payment: Payment::new(&options),
            token: Token::new(&options)
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error)
}