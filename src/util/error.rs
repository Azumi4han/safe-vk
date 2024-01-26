use crate::Shape;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Dimension index {dim} exceeds the maximum allowed shape dimensions (5x10) for shape {shape:?}")]
    DimOutOfRange { shape: Shape, dim: i32 },

    #[error("Expected status success (1) but got uknown status {status}")]
    EventAnswerUnkownStatus { status: i8 },

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] SerdeJsonError),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] ReqwestError),
}

pub type Result<T> = std::result::Result<T, Error>;
