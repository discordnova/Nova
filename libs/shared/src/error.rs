use config::ConfigError;
use std::{fmt::Debug, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenericError {
    #[error("invalid configuration")]
    InvalidConfiguration(#[from] ConfigError),

    #[error("invalid parameter `{0}`")]
    InvalidParameter(String),

    #[error("step `{0}` failed")]
    StepFailed(String),

    #[error("io error")]
    Io(#[from] io::Error),
}
