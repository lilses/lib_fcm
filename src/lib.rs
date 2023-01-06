use macros_make_error::make_error2;
use macros_make_model::make_model22;
use serde::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

make_error2!(FcmError);

make_model22!(
    QFcmMessage,
    IFcmMessage,
    OFcmMessage,
    fcm_message,
    message: i8,
    fcm_id: String
);

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Message {
    TransferwisePayment,
}

#[derive(Debug)]
pub struct FcmClient {
    pub client: reqwest::Client,
    url: String,
}

pub fn make_client(url: &str) -> FcmClient {
    let client = reqwest::Client::new();
    FcmClient {
        client,
        url: url.to_string(),
    }
}

pub async fn send_message(
    fmc_client: &FcmClient,
    message: &IFcmMessage,
) -> Result<QFcmMessage, FcmError> {
    fmc_client
        .client
        .post(&fmc_client.url)
        .json::<IFcmMessage>(&message)
        .send()
        .await
        .map_err(FcmError::from_general)?
        .error_for_status()
        .map_err(FcmError::from_general)?
        .json::<QFcmMessage>()
        .await
        .map_err(FcmError::from_general)
}
