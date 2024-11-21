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


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderEvent {
    pub order_ref: String,
    pub direction: String,
    pub offset: String,
    pub price: f64,
    pub volume_total_original: u32,   
    pub submit_status: String,
    pub order_type: String,
    pub sys_id: String,
    pub status: String,
    pub volume_traded: u32,
    pub volume_total: u32,
    pub status_msg: String,
    pub symbol: String,
    pub request_id: i32,
    pub invest_unit_id : String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionEvent {
    pub symbol : String,
    pub position: u32,
    pub today_position: u32,
    pub direction: String,
    pub cost: f64,
    pub cost_offset: f64,
    pub trading_day: String,
    pub invest_unit_id : String,
}