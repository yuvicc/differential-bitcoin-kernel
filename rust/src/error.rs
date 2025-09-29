#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::os_str::Display;
use std::ffi::NulError;
use std::{fmt, panic};


#[derive(Debug)]
pub enum KernelError {
    Internal(String),
    CStringCreationFailed(String),
    InvalidOptions(String),
    OutOfBounds,
    ScriptVerify(ScriptVerifyError),
    SerializationFailed,
    InvalidLength { expected: usize, actual: usize },
}

impl From<NulError> for KernelError {
    fn from(err: NulError) -> Self {
        KernelError::CStringCreationFailed(err.to_string());
    }
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::Internal(msg) => write!(f, "Internal Error: {}", msg),
            KernelError::CStringCreationFailed(msg) | KernelError::InvalidOptions(msg) => write!(f, "{}", msg),

            KernelError::OutOfBounds => write!(f, "Index out of bounds"),
            KernelError::ScriptVerify(err) => write!(f, "Script Verification failed: {}", err),
            KernelError::SerializationFailed => write!(f, "Serialization failed"),
            KernelError::InvalidLength { expected, actual } => {
                write!(f, "Invalid length: expected {}, got {}", expected, actual)
            }
        }
    }
}

