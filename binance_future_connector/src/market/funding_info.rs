use crate::http::{request::Request, Method};


/// `GET /fapi/v1/fundingInfo`
/// "Path /fapi/v1/fundingInfo, Method GET is invalid"
///
/// Query funding rate info for symbols that had FundingRateCap/ FundingRateFloor / fundingIntervalHours adjustment
/// ```
pub struct FundingInfo {
}

impl FundingInfo {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl From<FundingInfo> for Request {
    fn from(_: FundingInfo) -> Request {
        let params = vec![
        ];

        Request {
            path: "/fapi/v1/fundingInfo".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
        }
    }
}
