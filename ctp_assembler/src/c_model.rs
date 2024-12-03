use std::ffi::CString;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServiceResult<T: Serialize> {
    pub error_code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl <T: Serialize> ServiceResult<T> {
    pub fn new(error_code: i32, message: &str, data: Option<T>) -> Self {
        ServiceResult {
            error_code,
            message: message.to_string(),
            data,
        }
    }

    pub fn to_c_json(&self) -> Box<CString> {
        let json = serde_json::to_string(&self).unwrap();
        Box::new(CString::new(json).unwrap())
    }
}
