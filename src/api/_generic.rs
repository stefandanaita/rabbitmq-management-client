use http::StatusCode;
use reqwest::Response;
use serde::de::DeserializeOwned;
use crate::errors::{RabbitMqApiError, RabbitMqClientError};

pub async fn handle_response<T>(response: Response) -> Result<T, RabbitMqClientError>
where
    T: DeserializeOwned,
{
    let status = response.status();

    if status.is_success() {
       return match response.json::<T>().await {
            Ok(response) => Ok(response),
            Err(e) => Err(RabbitMqClientError::ParsingError(e)),
        }
    }

    if status.eq(&StatusCode::UNAUTHORIZED) {
        return Err(RabbitMqClientError::Unauthorized);
    }

    Err(RabbitMqClientError::ApiError(RabbitMqApiError {
        code: status,
        text: response.text().await.map_err(|e| RabbitMqClientError::ParsingError(e))?,
    }))
}