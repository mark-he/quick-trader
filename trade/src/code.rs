pub const DIRECTION_LONG: &str = "LONG"; //多
pub const DIRECTION_SHORT: &str = "SHORT"; //空

pub const ORDER_TYPE_LIMIT : &str = "LIMIT"; //限价
pub const ORDER_TYPE_MARKET : &str = "MARKET"; //市价
pub const ORDER_TYPE_FAK : &str = "FAK"; //立即成交任意，然后撤销
pub const ORDER_TYPE_FOK : &str = "FOK"; //立即成交全部，否则撤销

pub const OFFSET_OPEN : &str = "OPEN"; //开
pub const OFFSET_CLOSE : &str = "CLOSE"; //平

pub const ORDER_SUBMIT_INSERT_SUBMITTED : &str = "INSERT_SUBMITTED"; //下单提交
pub const ORDER_SUBMIT_CANCEL_SUBMITTED : &str = "CANCEL_SUBMITTED"; //撤单提交
pub const ORDER_SUBMIT_MODIFY_SUBMITTED : &str = "MODIFY_SUBMITTED"; //改单提交
pub const ORDER_SUBMIT_ACCEPTED_SUBMITTED : &str = "ACCEPTED_SUBMITTED"; //已接受
pub const ORDER_SUBMIT_INSERT_REJECTED : &str = "INSERT_REJECTED"; //下单拒绝
pub const ORDER_SUBMIT_CANCEL_REJECTED : &str = "CANCEL_REJECTED"; //撤回拒绝
pub const ORDER_SUBMIT_MODIFY_REJECTED : &str = "MODIFY_REJECTED"; //改单拒绝

pub const ORDER_STATUS_ALL_TRADED : &str = "ALL_TRADED";//全部成交
pub const ORDER_STATUS_PART_TRADED : &str = "PART_TRADED_QUEUEING";//部分成交还在队列中
pub const ORDER_STATUS_NO_TRADED : &str = "NO_TRADED_QUEUEING";//未成交还在队列中
pub const ORDER_STATUS_CANCELLED : &str = "CANCELLED";//撤单