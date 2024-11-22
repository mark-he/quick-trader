
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use libctp_sys::*;

/*
pub const THOST_FTDC_OST_AllTraded: u8 = 48u8;
pub const THOST_FTDC_OST_PartTradedQueueing: u8 = 49u8;
pub const THOST_FTDC_OST_PartTradedNotQueueing: u8 = 50u8;
pub const THOST_FTDC_OST_NoTradeQueueing: u8 = 51u8;
pub const THOST_FTDC_OST_NoTradeNotQueueing: u8 = 52u8;
pub const THOST_FTDC_OST_Canceled: u8 = 53u8;
pub const THOST_FTDC_OST_Unknown: u8 = 97u8;
*/
pub const ORDER_STATUS_ALL_TRADED : CtpCode = CtpCode {code: "ALL_TRADED", ctp_code: THOST_FTDC_OST_AllTraded};//全部成交
pub const ORDER_STATUS_PART_TRADED_QUEUEING : CtpCode = CtpCode {code: "PART_TRADED_QUEUEING", ctp_code : THOST_FTDC_OST_PartTradedQueueing};//部分成交还在队列中
pub const ORDER_STATUS_PART_TRADED_NOT_QUEUEING : CtpCode = CtpCode {code: "PART_TRADED_NOT_QUEUEING", ctp_code : THOST_FTDC_OST_PartTradedNotQueueing};//部分成交不在队列中
pub const ORDER_STATUS_NO_TRADED_QUEUEING : CtpCode = CtpCode {code: "NO_TRADED_QUEUEING", ctp_code : THOST_FTDC_OST_NoTradeQueueing};//未成交还在队列中
pub const ORDER_STATUS_NO_TRADED_NOT_QUEUEING : CtpCode = CtpCode {code: "NO_TRADED_NOT_QUEUEING", ctp_code : THOST_FTDC_OST_NoTradeNotQueueing};//未成交不在队列中
pub const ORDER_STATUS_CANCELLED : CtpCode = CtpCode {code: "CANCELLED", ctp_code : THOST_FTDC_OST_Canceled};//撤单
pub const ORDER_STATUS_UNKNOWN : CtpCode = CtpCode {code: "UNKNOWN", ctp_code : THOST_FTDC_OST_Unknown};//提交中
/*
pub const THOST_FTDC_D_Buy: u8 = 48u8;
pub const THOST_FTDC_D_Sell: u8 = 49u8;
*/
pub const DIRECTION_LONG : CtpCode = CtpCode {code : "LONG", ctp_code : THOST_FTDC_D_Buy}; //多
pub const DIRECTION_SHORT : CtpCode = CtpCode {code : "SHORT", ctp_code : THOST_FTDC_D_Sell}; //空
/*
pub const THOST_FTDC_PD_Net: u8 = 49u8;
pub const THOST_FTDC_PD_Long: u8 = 50u8;
pub const THOST_FTDC_PD_Short: u8 = 51u8;
*/
pub const POSITION_DIRECTION_NET : CtpCode = CtpCode {code : "NET", ctp_code : THOST_FTDC_PD_Net}; //净
pub const POSITION_DIRECTION_LONG : CtpCode = CtpCode {code : "LONG", ctp_code :THOST_FTDC_PD_Long}; //多
pub const POSITION_DIRECTION_SHORT : CtpCode = CtpCode {code : "SHORT", ctp_code : THOST_FTDC_PD_Short}; //空
/*
pub const THOST_FTDC_OPT_AnyPrice: u8 = 49u8;
pub const THOST_FTDC_OPT_LimitPrice: u8 = 50u8;
pub const THOST_FTDC_OPT_BestPrice: u8 = 51u8;
pub const THOST_FTDC_OPT_LastPrice: u8 = 52u8;

pub const THOST_FTDC_TC_IOC: u8 = 49u8;
pub const THOST_FTDC_TC_GFS: u8 = 50u8;
pub const THOST_FTDC_TC_GFD: u8 = 51u8;
pub const THOST_FTDC_TC_GTD: u8 = 52u8;
pub const THOST_FTDC_TC_GTC: u8 = 53u8;
pub const THOST_FTDC_TC_GFA: u8 = 54u8;
pub const THOST_FTDC_VC_AV: u8 = 49u8;
pub const THOST_FTDC_VC_MV: u8 = 50u8;
pub const THOST_FTDC_VC_CV: u8 = 51u8;
*/
pub struct OrderType {
    pub price_type: u8,
    pub time_condition: u8,
    pub volume_condition: u8,
}

impl FromStr for OrderType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LIMIT" => Ok(OrderType {price_type: THOST_FTDC_OPT_LimitPrice, time_condition: THOST_FTDC_TC_GFD, volume_condition: THOST_FTDC_VC_AV}),
            "MARKET" => Ok(OrderType {price_type: THOST_FTDC_OPT_AnyPrice, time_condition: THOST_FTDC_TC_IOC, volume_condition: THOST_FTDC_VC_AV}),
            "FAK" => Ok(OrderType {price_type: THOST_FTDC_OPT_LimitPrice, time_condition: THOST_FTDC_TC_IOC, volume_condition: THOST_FTDC_VC_AV}),
            "FOK" => Ok(OrderType {price_type: THOST_FTDC_OPT_LimitPrice, time_condition: THOST_FTDC_TC_IOC, volume_condition: THOST_FTDC_VC_CV}),
            _ => Err("Invalid OrderType".to_string()),
        }
    }
}

#[allow(non_upper_case_globals)]
impl ToString for OrderType {
    fn to_string(&self) -> String {
        match (self.price_type, self.time_condition, self.volume_condition) {
            (THOST_FTDC_OPT_LimitPrice, THOST_FTDC_TC_GFD, THOST_FTDC_VC_AV) => {
                return "LIMIT".to_string();
            },
            (THOST_FTDC_OPT_AnyPrice, THOST_FTDC_TC_IOC, THOST_FTDC_VC_AV) => {
                return "MARKET".to_string();
            },
            (THOST_FTDC_OPT_LimitPrice, THOST_FTDC_TC_IOC, THOST_FTDC_VC_AV) => {
                return "FAK".to_string();
            },
            (THOST_FTDC_OPT_LimitPrice, THOST_FTDC_TC_IOC, THOST_FTDC_VC_CV) => {
                return "FOK".to_string();
            },
            _ => {
                return "Unknown".to_string();
            },
        }
    }
}

/*
pub const THOST_FTDC_OF_Open: u8 = 48u8;
pub const THOST_FTDC_OF_Close: u8 = 49u8;
pub const THOST_FTDC_OF_ForceClose: u8 = 50u8;
pub const THOST_FTDC_OF_CloseToday: u8 = 51u8;
pub const THOST_FTDC_OF_CloseYesterday: u8 = 52u8;
pub const THOST_FTDC_OF_ForceOff: u8 = 53u8;
pub const THOST_FTDC_OF_LocalForceClose: u8 = 54u8;
*/
pub const OFFSET_OPEN : CtpCode = CtpCode {code : "OPEN", ctp_code : THOST_FTDC_OF_Open}; //开
pub const OFFSET_CLOSE : CtpCode = CtpCode {code : "CLOSE", ctp_code : THOST_FTDC_OF_Close}; //平
pub const OFFSET_CLOSETODAY : CtpCode = CtpCode {code : "CLOSETODAY", ctp_code : THOST_FTDC_OF_CloseToday}; //平今
pub const OFFSET_CLOSEYESTERDAY : CtpCode = CtpCode {code : "CLOSEYESTERDAY", ctp_code : THOST_FTDC_OF_CloseYesterday}; //平昨

/*
pub const THOST_FTDC_OSS_InsertSubmitted: u8 = 48u8;
pub const THOST_FTDC_OSS_CancelSubmitted: u8 = 49u8;
pub const THOST_FTDC_OSS_ModifySubmitted: u8 = 50u8;
pub const THOST_FTDC_OSS_Accepted: u8 = 51u8;
pub const THOST_FTDC_OSS_InsertRejected: u8 = 52u8;
pub const THOST_FTDC_OSS_CancelRejected: u8 = 53u8;
pub const THOST_FTDC_OSS_ModifyRejected: u8 = 54u8;
*/
pub const ORDER_SUBMIT_INSERT_SUBMITTED : CtpCode = CtpCode {code: "INSERT_SUBMITTED", ctp_code : THOST_FTDC_OSS_InsertSubmitted};//下单提交
pub const ORDER_SUBMIT_CANCEL_SUBMITTED : CtpCode = CtpCode {code: "CANCEL_SUBMITTED", ctp_code : THOST_FTDC_OSS_CancelSubmitted};//撤单提交
pub const ORDER_SUBMIT_MODIFY_SUBMITTED : CtpCode = CtpCode {code: "MODIFY_SUBMITTED", ctp_code : THOST_FTDC_OSS_ModifySubmitted};//改单提交
pub const ORDER_SUBMIT_ACCEPTED_SUBMITTED : CtpCode = CtpCode {code: "ACCEPTED_SUBMITTED", ctp_code : THOST_FTDC_OSS_Accepted};//已接受
pub const ORDER_SUBMIT_INSERT_REJECTED : CtpCode = CtpCode {code: "INSERT_REJECTED", ctp_code : THOST_FTDC_OSS_InsertRejected};//下单拒绝
pub const ORDER_SUBMIT_CANCEL_REJECTED : CtpCode = CtpCode {code: "CANCEL_REJECTED", ctp_code : THOST_FTDC_OSS_CancelRejected};//撤回拒绝
pub const ORDER_SUBMIT_MODIFY_REJECTED : CtpCode = CtpCode {code: "MODIFY_REJECTED", ctp_code : THOST_FTDC_OSS_ModifyRejected};//改单拒绝

pub struct CtpCode {
    pub code : &'static str, 
    pub ctp_code : u8,
}

pub static ORDER_SUBMIT: LazyLock<Arc<HashMap<String, u8>>> = LazyLock::new(|| {
    let map : HashMap<String, u8> = ctp_code_array_to_hashmap(&[
        &ORDER_SUBMIT_INSERT_SUBMITTED, 
        &ORDER_SUBMIT_CANCEL_SUBMITTED, 
        &ORDER_SUBMIT_MODIFY_SUBMITTED, 
        &ORDER_SUBMIT_ACCEPTED_SUBMITTED,
        &ORDER_SUBMIT_INSERT_REJECTED,
        &ORDER_SUBMIT_CANCEL_REJECTED,
        &ORDER_SUBMIT_MODIFY_REJECTED,
    ]);
    Arc::new(map)
});

//委托状态映射
pub static ORDER_STATUS: LazyLock<Arc<HashMap<String, u8>>> = LazyLock::new(|| {
    let map : HashMap<String, u8> = ctp_code_array_to_hashmap(&[
        &ORDER_STATUS_ALL_TRADED, 
        &ORDER_STATUS_PART_TRADED_QUEUEING, 
        &ORDER_STATUS_PART_TRADED_NOT_QUEUEING, 
        &ORDER_STATUS_NO_TRADED_QUEUEING,
        &ORDER_STATUS_NO_TRADED_NOT_QUEUEING,
        &ORDER_STATUS_CANCELLED,
        &ORDER_STATUS_UNKNOWN,
    ]);
    Arc::new(map)
});

pub static POSITION_DIRECTION: LazyLock<Arc<HashMap<String, u8>>> = LazyLock::new(|| {
    let map : HashMap<String, u8> = ctp_code_array_to_hashmap(&[
        &POSITION_DIRECTION_NET,
        &POSITION_DIRECTION_LONG,
        &POSITION_DIRECTION_SHORT,
    ]);
    Arc::new(map)
});

pub static DIRECTION: LazyLock<Arc<HashMap<String, u8>>> = LazyLock::new(|| {
    let map : HashMap<String, u8> = ctp_code_array_to_hashmap(&[
        &DIRECTION_LONG,
        &DIRECTION_SHORT,
    ]);
    Arc::new(map)
});

pub static OFFSET: LazyLock<Arc<HashMap<String, u8>>> = LazyLock::new(|| {
    let map : HashMap<String, u8> = ctp_code_array_to_hashmap(&[
        &OFFSET_OPEN,
        &OFFSET_CLOSE,
        &OFFSET_CLOSETODAY,
        &OFFSET_CLOSEYESTERDAY,
    ]);
    Arc::new(map)
});

pub static ORDER_SUBMIT_REV:  LazyLock<Arc<HashMap<u8, String>>> = LazyLock::new(|| {
    let map : HashMap<u8, String> = reverse_hashmap(&ORDER_SUBMIT);
    Arc::new(map)
});

pub static ORDER_STATUS_REV:  LazyLock<Arc<HashMap<u8, String>>> = LazyLock::new(|| {
    let map : HashMap<u8, String> = reverse_hashmap(&ORDER_STATUS);
    Arc::new(map)
});

pub static DIRECTION_REV: LazyLock<Arc<HashMap<u8, String>>> = LazyLock::new(|| {
    let map : HashMap<u8, String> = reverse_hashmap(&DIRECTION);
    Arc::new(map)
});

pub static POSITION_DIRECTION_REV: LazyLock<Arc<HashMap<u8, String>>> = LazyLock::new(|| {
    let map : HashMap<u8, String> = reverse_hashmap(&POSITION_DIRECTION);
    Arc::new(map)
});

pub static OFFSET_REV:  LazyLock<Arc<HashMap<u8, String>>> = LazyLock::new(|| {
    let map : HashMap<u8, String> = reverse_hashmap(&OFFSET);
    Arc::new(map)
});

fn ctp_code_array_to_hashmap(arr: &[&CtpCode]) -> HashMap<String, u8> {
    let mut map = HashMap::new();
    for item in arr {
        map.insert(item.code.to_string(), item.ctp_code);
    }
    map
}

fn reverse_hashmap(map: &HashMap<String, u8>) -> HashMap<u8, String> {
    let mut reversed_map = HashMap::new();
    for (key, value) in map {
        reversed_map.insert(value.clone(), key.clone());
    }
    reversed_map
}
