use crate::errors::{RabbitMqApiError, RabbitMqClientError};
use http::StatusCode;
use reqwest::Response;
use serde::de::DeserializeOwned;

#[tracing::instrument(skip(response))]
pub async fn handle_response<T>(response: Response) -> Result<T, RabbitMqClientError>
where
    T: DeserializeOwned,
{
    let status = response.status();

    let text = response
        .text()
        .await
        .map_err(RabbitMqClientError::ResponseError)?;

    if status.is_success() {
        match serde_json::from_str(&text) {
            Ok(data) => Ok(data),
            Err(e) => Err(RabbitMqClientError::JSONError(e)),
        }
    } else {
        Err(map_error(status, text))
    }
}

#[tracing::instrument(skip(response))]
pub async fn handle_empty_response(response: Response) -> Result<(), RabbitMqClientError> {
    let status = response.status();

    if status.is_success() {
        Ok(())
    } else {
        let text = response
            .text()
            .await
            .map_err(RabbitMqClientError::ResponseError)?;

        Err(map_error(status, text))
    }
}

fn map_error(status: StatusCode, text: String) -> RabbitMqClientError {
    if status.eq(&StatusCode::UNAUTHORIZED) {
        return RabbitMqClientError::Unauthorized;
    }

    if status.eq(&StatusCode::NOT_FOUND) {
        return RabbitMqClientError::NotFound(text);
    }

    RabbitMqClientError::ApiError(RabbitMqApiError { code: status, text })
}
