use crate::response::api::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FaucetError {
    #[error("Invalid POW")]
    InvalidPoW,
    #[error("Invalid tag")]
    InvalidProof,
    #[error("POW update already seen")]
    DuplicateChallenge,
    #[error("Invalid address")]
    InvalidAddress,
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Chain didn't start yet")]
    ChainNotStarted,
    #[error("Faucet out of balance, please try again later")]
    FaucetOutOfBalance,
    #[error("Error while sending transfer: {0}")]
    SdkError(String),
    // #[error("Withdraw limit must be less then {0}")]
    // InvalidWithdrawLimit(u64),
    // #[error("Public key {0} does not belong to a shielded expedition player")]
    // NotPlayer(String),
    // #[error("Please come back after 24 hours if you want to receive more")]
    // TooManyRequests,
    #[error("Please come back after a few minutes")]
    TooManyRequests,
}

impl IntoResponse for FaucetError {
    fn into_response(self) -> Response {
        let status_code = match self {
            FaucetError::InvalidPoW => StatusCode::BAD_REQUEST,
            FaucetError::InvalidProof => StatusCode::FORBIDDEN,
            FaucetError::DuplicateChallenge => StatusCode::CONFLICT,
            FaucetError::InvalidAddress => StatusCode::BAD_REQUEST,
            FaucetError::ChainNotStarted => StatusCode::BAD_REQUEST,
            // FaucetError::InvalidWithdrawLimit(_) => StatusCode::BAD_REQUEST,
            FaucetError::FaucetOutOfBalance => StatusCode::CONFLICT,
            FaucetError::SdkError(_) => StatusCode::BAD_REQUEST,
            // FaucetError::NotPlayer(_) => StatusCode::BAD_REQUEST,
            FaucetError::TooManyRequests => StatusCode::BAD_REQUEST,
            FaucetError::InvalidPublicKey => StatusCode::BAD_REQUEST,
            FaucetError::InvalidSignature => StatusCode::BAD_REQUEST,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
