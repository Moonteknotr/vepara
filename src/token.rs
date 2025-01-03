use std::future::Future;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::ClientOptions;

pub struct Token {
    options: ClientOptions
}

impl Token {
    pub fn new(options: ClientOptions) -> Self {
        Token {
            options
        }
    }

    pub async fn payment_2d(&self, body: &Payment2DBody) -> Result<Payment2DResponse, super::Error> {
        let url = format!("{}/api/paySmart2D", self.options.api_base);
        println!("{:?}", url);
        let request = self.options.reqwest_client.clone().post(&url).json(body).send().await;
        let response = match request {
            Ok(response) => response,
            Err(e) => return Err(super::Error::ReqwestError(e))
        };
        let body = match response.json::<Value>().await {
            Ok(body) => body,
            Err(e) => return Err(super::Error::ReqwestError(e))
        };
        println!("{:?}", body);
        let body = match serde_json::from_value::<Payment2DResponse>(body) {
            Ok(body) => body,
            Err(e) => return Err(super::Error::SerdeError(e))
        };
        Ok(body)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    name: String,
    quantity: i32,
    price: f64,
    description: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment2DBody {
    cc_holder_name: String,
    cc_no: String,
    expiry_year: i32,
    expiry_month: i32,
    cvv: Option<i32>,
    currency_code: String,
    installments_number: i32,
    invoice_id: String,
    invoice_description: String,
    name: String,
    surname: String,
    total: f64,
    merchant_key: String,
    items: Vec<Item>,
    hash_key: String,
    vpos_type: Option<String>,
    identity_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Payment2DSuccessData {
    pub order_no: String,
    pub order_id: String,
    pub invoice_id: String,
    pub credit_card_no: String,
    pub transaction_type: String,
    pub payment_status: i64,
    pub payment_method: i64,
    pub error_code: i64,
    pub error: String,
    pub auth_code: i64,
    pub merchant_commission: f64,
    pub user_commission: i64,
    pub merchant_commission_percentage: f64,
    pub merchant_commission_fixed: i64,
    pub payment_reason_code: String,
    pub payment_reason_code_detail: String,
    pub hash_key: String,
    pub original_bank_error_code: String,
    pub original_bank_error_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Payment2DSuccessResponse {
    pub status_code: i64,
    pub status_description: String,
    pub data: Payment2DSuccessData,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payment2DInvalidHashData {
    pub invoice_id: String,
    pub order_no: String,
    pub order_id: String,
    pub credit_card_no: String,
    pub transaction_type: String,
    pub payment_status: i64,
    pub payment_method: i64,
    pub error_code: i64,
    pub error: String,
    pub auth_code: String,
    pub hash_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payment2DInvalidSubData {
    pub status_code: i64,
    pub status_description: String,
    pub data: Payment2DInvalidHashData,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payment2DInvalidHashResponse {
    pub data: Payment2DInvalidSubData,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payment2DErrorsBody {
    pub merchant_key: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payment2DErrorResponse {
    pub message: String,
    pub errors: Payment2DErrorsBody,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Payment2DResponse {
    Success(Payment2DSuccessResponse),
    InvalidHash(Payment2DInvalidHashResponse),
    Error(Payment2DErrorResponse),
}


#[cfg(test)]
mod tests {
    use crate::ClientOptionsBuilder;
    use super::*;

    #[tokio::test]
    async fn test_payment_2d() {
        let options = ClientOptionsBuilder::default().enable_dev_mode().build();
        let token = Token::new(options);
        let response = token.payment_2d(&Payment2DBody {
            cc_holder_name: "John Doe".to_string(),
            cc_no: "1234567890123456".to_string(),
            expiry_year: 2023,
            expiry_month: 12,
            cvv: Some(123),
            currency_code: "TRY".to_string(),
            installments_number: 1,
            invoice_id: "INV-123456".to_string(),
            invoice_description: "Test invoice".to_string(),
            name: "John".to_string(),
            surname: "Doe".to_string(),
            total: 100.0,
            merchant_key: "test".to_string(),
            items: vec![Item {
                name: "Test Item".to_string(),
                quantity: 1,
                price: 100.0,
                description: "Test item description".to_string()
            }],
            hash_key: "test".to_string(),
            vpos_type: None,
            identity_number: None
        }).await;
        println!("{:?}", response);
    }
}