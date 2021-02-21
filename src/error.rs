extern crate reqwest;

use std::fmt;
use crate::structs::ErrorResponse;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    CoffeeHouseError(ErrorResponse),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReqwestError(err) => write!(f, "ReqwestError: {}", err),
            Self::CoffeeHouseError(err) => write!(f, "CoffeeHouseError: {:?}", err),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(err)
    }
}

impl From<ErrorResponse> for Error {
    fn from(err: ErrorResponse) -> Self {
        Self::CoffeeHouseError(err)
    }
}
