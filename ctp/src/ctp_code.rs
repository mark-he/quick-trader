
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use trade::code::{self};

pub const ORDER_STATUS_ALL_TRADED : CtpCode = CtpCode {code: code::ORDER_STATUS_ALL_TRADED, ctp_code: "0"};//全部成交
pub const ORDER_STATUS_PART_TRADED_QUEUEING : CtpCode = CtpCode {code: code::ORDER_STATUS_PART_TRADED, ctp_code : "1"};//部分成交还在队列中
pub const ORDER_STATUS_PART_TRADED_NOT_QUEUEING : CtpCode = CtpCode {code: "PART_TRADED_NOT_QUEUEING", ctp_code : "2"};//部分成交不在队列中
pub const ORDER_STATUS_NO_TRADED_QUEUEING : CtpCode = CtpCode {code: code::ORDER_STATUS_NO_TRADED, ctp_code : "3"};//未成交还在队列中
pub const ORDER_STATUS_NO_TRADED_NOT_QUEUEING : CtpCode = CtpCode {code: "NO_TRADED_NOT_QUEUEING", ctp_code : "4"};//未成交不在队列中
pub const ORDER_STATUS_CANCELLED : CtpCode = CtpCode {code: code::ORDER_STATUS_CANCELLED, ctp_code : "5"};//撤单
pub const ORDER_STATUS_UNKNOWN : CtpCode = CtpCode {code: "UNKNOWN", ctp_code : "a"};//提交中

pub const DIRECTION_LONG : CtpCode = CtpCode {code : code::DIRECTION_LONG, ctp_code : "0"}; //多
pub const DIRECTION_SHORT : CtpCode = CtpCode {code : code::DIRECTION_SHORT, ctp_code : "1"}; //空

pub const ORDER_TYPE_LIMIT : CtpCode = CtpCode {code : code::ORDER_TYPE_LIMIT, ctp_code : "2,3,1"}; //限价
pub const ORDER_TYPE_MARKET : CtpCode = CtpCode {code : code::ORDER_TYPE_MARKET, ctp_code : "1,3,1"}; //市价
pub const ORDER_TYPE_FAK : CtpCode = CtpCode {code : code::ORDER_TYPE_FAK, ctp_code : "2,1,1"}; //立即成交任意，然后撤销
pub const ORDER_TYPE_FOK : CtpCode = CtpCode {code : code::ORDER_TYPE_FOK, ctp_code : "2,1,3"}; //立即成交全部，否则撤销

pub const OFFSET_OPEN : CtpCode = CtpCode {code : code::OFFSET_OPEN, ctp_code : "0"}; //开
pub const OFFSET_CLOSE : CtpCode = CtpCode {code : code::OFFSET_CLOSE, ctp_code : "1"}; //多
pub const OFFSET_CLOSETODAY : CtpCode = CtpCode {code : "CLOSETODAY", ctp_code : "3"}; //平今
pub const OFFSET_CLOSEYESTERDAY : CtpCode = CtpCode {code : "CLOSEYESTERDAY", ctp_code : "4"}; //平昨

pub const ORDER_SUBMIT_INSERT_SUBMITTED : CtpCode = CtpCode {code: code::ORDER_SUBMIT_INSERT_SUBMITTED, ctp_code : "0"};//下单提交
pub const ORDER_SUBMIT_CANCEL_SUBMITTED : CtpCode = CtpCode {code: code::ORDER_SUBMIT_CANCEL_SUBMITTED, ctp_code : "1"};//撤单提交
pub const ORDER_SUBMIT_MODIFY_SUBMITTED : CtpCode = CtpCode {code: code::ORDER_SUBMIT_MODIFY_SUBMITTED, ctp_code : "2"};//改单提交
pub const ORDER_SUBMIT_ACCEPTED_SUBMITTED : CtpCode = CtpCode {code: code::ORDER_SUBMIT_ACCEPTED_SUBMITTED, ctp_code : "3"};//已接受
pub const ORDER_SUBMIT_INSERT_REJECTED : CtpCode = CtpCode {code: code::ORDER_SUBMIT_INSERT_REJECTED, ctp_code : "4"};//下单拒绝
pub const ORDER_SUBMIT_CANCEL_REJECTED : CtpCode = CtpCode {code: code::ORDER_SUBMIT_CANCEL_REJECTED, ctp_code : "5"};//撤回拒绝
pub const ORDER_SUBMIT_MODIFY_REJECTED : CtpCode = CtpCode {code: code::ORDER_SUBMIT_MODIFY_REJECTED, ctp_code : "6"};//改单拒绝

pub struct CtpCode {
    pub code : &'static str, 
    pub ctp_code : &'static str,
}

pub static ORDER_SUBMIT: LazyLock<Arc<HashMap<String, String>>> = LazyLock::new(|| {
    let map : HashMap<String, String> = ctp_code_array_to_hashmap(&[
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
pub static ORDER_STATUS: LazyLock<Arc<HashMap<String, String>>> = LazyLock::new(|| {
    let map : HashMap<String, String> = ctp_code_array_to_hashmap(&[
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

pub static DIRECTION: LazyLock<Arc<HashMap<String, String>>> = LazyLock::new(|| {
    let map : HashMap<String, String> = ctp_code_array_to_hashmap(&[
        &DIRECTION_LONG,
        &DIRECTION_SHORT,
    ]);
    Arc::new(map)
});

pub static ORDER_TYPE:  LazyLock<Arc<HashMap<String, String>>> = LazyLock::new(|| {
    let map : HashMap<String, String> = ctp_code_array_to_hashmap(&[
        &ORDER_TYPE_LIMIT,
        &ORDER_TYPE_MARKET,
        &ORDER_TYPE_FAK,
        &ORDER_TYPE_FOK,
    ]);
    Arc::new(map)
});

pub static OFFSET: LazyLock<Arc<HashMap<String, String>>> = LazyLock::new(|| {
    let map : HashMap<String, String> = ctp_code_array_to_hashmap(&[
        &OFFSET_OPEN,
        &OFFSET_CLOSE,
        &OFFSET_CLOSETODAY,
        &OFFSET_CLOSEYESTERDAY,
    ]);
    Arc::new(map)
});

pub static ORDER_SUBMIT_REV:  LazyLock<Arc<HashMap<String, String>>> = LazyLock::new(|| {
    let map : HashMap<String, String> = reverse_hashmap(&ORDER_SUBMIT);
    Arc::new(map)
});

pub static ORDER_STATUS_REV:  LazyLock<Arc<HashMap<String, String>>> = LazyLock::new(|| {
    let map : HashMap<String, String> = reverse_hashmap(&ORDER_STATUS);
    Arc::new(map)
});

pub static DIRECTION_REV: LazyLock<Arc<HashMap<String, String>>> = LazyLock::new(|| {
    let map : HashMap<String, String> = reverse_hashmap(&DIRECTION);
    Arc::new(map)
});

pub static ORDER_TYPE_REV:  LazyLock<Arc<HashMap<String, String>>> = LazyLock::new(|| {
    let map : HashMap<String, String> = reverse_hashmap(&ORDER_TYPE);
    Arc::new(map)
});

pub static OFFSET_REV:  LazyLock<Arc<HashMap<String, String>>> = LazyLock::new(|| {
    let map : HashMap<String, String> = reverse_hashmap(&OFFSET);
    Arc::new(map)
});

pub struct OrderType {
    pub price_type: char,
    pub time_condition: char,
    pub volume_condition: char,
}

impl OrderType {
    pub fn to_string(&self) -> String {
        format!("{},{},{}", self.price_type, self.time_condition, self.volume_condition)
    }

    pub fn from_string(s: &str) -> OrderType {
        let parts: Vec<&str> = s.split(',').collect();
        OrderType {
            price_type: parts[0].chars().next().unwrap(),
            time_condition: parts[1].chars().next().unwrap(),
            volume_condition: parts[2].chars().next().unwrap(),
        }
    }
}

fn ctp_code_array_to_hashmap(arr: &[&CtpCode]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for item in arr {
        map.insert(item.code.to_string(), item.ctp_code.to_string());
    }
    map
}

fn reverse_hashmap(map: &HashMap<String, String>) -> HashMap<String, String> {
    let mut reversed_map = HashMap::new();
    for (key, value) in map {
        reversed_map.insert(value.clone(), key.clone());
    }
    reversed_map
}
