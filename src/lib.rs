use std::sync::Arc;
use crate::payment::Payment;
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
    options: Arc<ClientOptions>,
    pub payment: Payment
}

impl Client {
    pub fn new(options: ClientOptions) -> Self {
        let options = Arc::new(options);
        Client {
            payment: Payment::new(&options),
            options,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_options() {
        let options = ClientOptionsBuilder::with_merchant_key("test".to_string()).build();
        println!("{:?}", options.api_base);
        assert_eq!(options.merchant_key.unwrap(), "test".to_string());

    }

    #[test]
    fn test_sub_client() {
        let options = ClientOptionsBuilder::with_merchant_key("test".to_string()).build();
        let client = Client::new(options);
    }
}

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error)
}