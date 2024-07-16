use std::io;
use thiserror::Error;
use url::ParseError;

use crate::client::APIError;

/// Default Error enum which provides translation between std error to different
/// error types
#[derive(Error, Debug)]
pub enum PorkbunnError {
    #[error("HTTP Request error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Failed during URL parsing")]
    URLParseError(#[from] ParseError),

    #[error("Failed during IO operation")]
    IOError(#[from] io::Error),
    //     backtrace: BackTrace
    // },
    #[error("Unsupported method ")]
    UnsupportedMethod,

    #[error("Failed during Serde operation")]
    SerdeError(#[from] serde_json::Error),

    #[error("Failed during parsing APIResponse: {message:?} and errors {errors:?}")]
    APIResponseError {
        errors: Vec<APIError>,
        message: String,
    },
}
