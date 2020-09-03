use serde::{
    Serialize
};

/// General API response object
///
/// Intended for use with JSON Responses
pub struct APIResponse<T: Serialize> {
    pub success: bool,
    pub payload: T
}