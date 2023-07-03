use gjson::Value;
use crate::error::CustomError;
use protos::matcher::MatchRequest;
use crate::detective;

pub fn any(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

pub fn credit_card(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

pub fn ssn(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

pub fn email(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

pub fn drivers_license(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

pub fn passport_id(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

pub fn vin_number(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

pub fn phone(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

// Intended to operate on the entire payload
pub fn serial_number(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

// Intended to operate on the entire payload
pub fn login(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

pub fn taxpayer_id(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

// Intended to operate on the entire payload
pub fn address(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

// Intended to operate on the entire payload
pub fn signature(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

// Intended to operate on the entire payload
pub fn geolocation(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

// Intended to operate on the entire payload
pub fn education(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

// Intended to operate on the entire payload
pub fn financial(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}

// Intended to operate on the entire payload
pub fn health(_request: &MatchRequest) -> Result<bool, CustomError> {
    Err(CustomError::Error("not implemented".to_string()))
}
