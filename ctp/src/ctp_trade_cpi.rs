#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use libctp_sys::*;
use trade::trade_server::{Order, Position, Wallet};
use std::collections::HashMap;
use std::os::raw::*;
use common::{c::*, msmc::Subscription};
use crate::model::{Session, SymbolInfo, ServerEvent};

use super::ctp_code::*;

pub struct Spi {
    pub subscription: Subscription<ServerEvent>,
    pub position_queue : HashMap<i32, Vec<Position>>,
}

impl Spi {
    pub fn new() -> Self {
        Spi { subscription: Subscription::<ServerEvent>::top(), position_queue: HashMap::new() }
    }

    fn handle_result<F>(subscription: &Subscription<ServerEvent>, request_id: i32, pRspInfo: *mut CThostFtdcRspInfoField, f: &mut F)
        where F: FnMut() {      
        if !pRspInfo.is_null() {
            let pRspInfo = unsafe { &mut *pRspInfo };
            if pRspInfo.ErrorID != 0 as c_int {
                subscription.send(&ServerEvent::Error(request_id,  c_char_to_gbk_string(pRspInfo.ErrorMsg.as_ptr())));
                return
            }
        }
        f();
    }

    fn convert_order(pRspInfo: *mut CThostFtdcOrderField) -> Order {
        let pRspInfo = unsafe { &mut *pRspInfo };
        let order_type = OrderType {
            price_type: pRspInfo.OrderPriceType as u8,
            time_condition: pRspInfo.TimeCondition as u8,
            volume_condition: pRspInfo.VolumeCondition as u8,
        };
        let order = Order {
            client_order_id: c_char_to_string(pRspInfo.OrderRef.as_ptr()),
            side:  DIRECTION_REV.as_ref().get(&(pRspInfo.Direction as u8)).unwrap().to_string(),
            offset: OFFSET_REV.as_ref().get(&(pRspInfo.CombOffsetFlag[0] as u8)).unwrap().to_string(),
            price: pRspInfo.LimitPrice,
            order_type: order_type.to_string(),
            total: pRspInfo.VolumeTotalOriginal as f64,   
            order_id: c_char_to_string(pRspInfo.OrderSysID.as_ptr()),
            status: ORDER_STATUS_REV.as_ref().get(&(pRspInfo.OrderStatus as u8)).unwrap().to_string(),
            traded: pRspInfo.VolumeTraded as f64,
            message: c_char_to_gbk_string(pRspInfo.StatusMsg.as_ptr()),
            symbol: c_char_to_string(pRspInfo.InstrumentID.as_ptr()),
            timestamp: 0,
        };
        order
    }

    fn convert_position(pRspInfo: *mut CThostFtdcInvestorPositionField) -> Position {
        let pRspInfo = unsafe { &mut *pRspInfo };
        let position = Position {
            symbol : c_char_to_string(pRspInfo.InstrumentID.as_ptr()),
            amount: pRspInfo.Position as f64,
            today_amount: pRspInfo.TodayPosition as f64,
            side: POSITION_DIRECTION_REV.as_ref().get(&(pRspInfo.PosiDirection as u8)).unwrap().to_string(),
            cost: pRspInfo.PositionCost,
            position_side: POSITION_DIRECTION_REV.as_ref().get(&(pRspInfo.PosiDirection as u8)).unwrap().to_string(),
        };
        position
    }

    fn convert_account(pRspInfo: *mut CThostFtdcTradingAccountField) -> Wallet {
        let pRspInfo = unsafe { &mut *pRspInfo };

        let account = Wallet {
            asset : c_char_to_string(pRspInfo.AccountID.as_ptr()),
            balance: pRspInfo.Balance,
            available_balance: pRspInfo.Available,
        };
        account
    }

    fn convert_session(pRspInfo: *mut CThostFtdcRspUserLoginField) -> Session {
        let pRspInfo = unsafe { &mut *pRspInfo };

        let session = Session {
            session_id: pRspInfo.SessionID,
            front_id: pRspInfo.FrontID,
            trading_day: c_char_to_string(pRspInfo.TradingDay.as_ptr()),
        };
        session
    }
    
    fn convert_symbol_info(pRspInfo: *mut CThostFtdcInstrumentField) -> SymbolInfo {
        let pRspInfo = unsafe { &mut *pRspInfo };

        let info = SymbolInfo {
            symbol: c_char_to_string(pRspInfo.InstrumentID.as_ptr()),
            margin_ratio: pRspInfo.LongMarginRatio,
            underlying_multiple: pRspInfo.UnderlyingMultiple,
            volume_multiple: pRspInfo.VolumeMultiple as f64,
            price_tick: pRspInfo.PriceTick,
        };
        info
    }
}

impl Rust_CThostFtdcTraderSpi_Trait for Spi {
    fn on_rsp_error(&mut self, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, _bIsLast: bool) { 
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
        });
    }

    fn on_err_rtn_order_insert(&mut self, pInputOrder: *mut CThostFtdcInputOrderField, pRspInfo: *mut CThostFtdcRspInfoField) { 
        let temp = unsafe { &mut *pInputOrder };
        Self::handle_result(&self.subscription, temp.RequestID, pRspInfo, &mut ||{
        });
    }

    fn on_err_rtn_order_action(&mut self, pOrderAction: *mut CThostFtdcOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField) { 
        let temp = unsafe { &mut *pOrderAction };
        Self::handle_result(&self.subscription, temp.RequestID, pRspInfo, &mut ||{
        });
    }

    fn on_err_rtn_exec_order_insert(&mut self, pInputExecOrder: *mut CThostFtdcInputExecOrderField, pRspInfo: *mut CThostFtdcRspInfoField) {
        let temp = unsafe { &mut *pInputExecOrder };
        Self::handle_result(&self.subscription, temp.RequestID, pRspInfo, &mut ||{
        });
    }

    fn on_err_rtn_exec_order_action(&mut self, pExecOrderAction: *mut CThostFtdcExecOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField) { 
        let temp = unsafe { &mut *pExecOrderAction };
        Self::handle_result(&self.subscription, temp.RequestID, pRspInfo, &mut ||{
        });
    }

    fn on_front_connected(&mut self) {
        self.subscription.send(&ServerEvent::Connected);
    }

    fn on_front_disconnected(&mut self, nReason: ::std::os::raw::c_int) {
        self.subscription.send(&ServerEvent::Disconnected(nReason)); 
    }

    fn on_heart_beat_warning(&mut self, nTimeLapse: ::std::os::raw::c_int) {
        self.subscription.send(&ServerEvent::HeartBeatWarning(nTimeLapse)); 
    }

    fn on_rsp_user_login(&mut self, _pRspUserLogin: *mut CThostFtdcRspUserLoginField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, _bIsLast: bool) { 
        //let pRspUserLogin = unsafe { &mut *pRspUserLogin };
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
            let session = Self::convert_session(_pRspUserLogin);
            self.subscription.send(&ServerEvent::UserLogin(session));
        });
    }

    fn on_rsp_user_logout(&mut self, _pUserLogout: *mut CThostFtdcUserLogoutField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, _bIsLast: bool) { 
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
            self.subscription.send(&ServerEvent::UserLogout);
        });
    }

    fn on_rsp_order_insert(&mut self, _pInputOrder: *mut CThostFtdcInputOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, _bIsLast: bool) { 
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
        });
    }

    fn on_rsp_order_action(&mut self, _pInputOrderAction: *mut CThostFtdcInputOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, _bIsLast: bool) { 
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
        });
    }

    fn on_rsp_settlement_info_confirm(&mut self, _pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, _bIsLast: bool) { 
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
            self.subscription.send(&ServerEvent::SettlementConfirmed);
        });
    }

    fn on_rtn_order(&mut self, pOrder: *mut CThostFtdcOrderField) {
        let ret = Self::convert_order(pOrder);
        let _ = self.subscription.send(&ServerEvent::OnOrder(ret));
    }

    fn on_rsp_qry_investor_position(&mut self, pInvestorPosition: *mut CThostFtdcInvestorPositionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) { 
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
            let position = Self::convert_position(pInvestorPosition);
            self.position_queue.entry(nRequestID).or_insert_with(Vec::new).push(position);
            if bIsLast {
                let mut ret = self.position_queue.remove(&nRequestID);
                if let Some(position_vec) = ret.take() {
                    self.subscription.send(&ServerEvent::PositionQuery(position_vec));
                }
            }
        });
    }

    fn on_rsp_qry_trading_account(&mut self, pTradingAccount: *mut CThostFtdcTradingAccountField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) { 
        if bIsLast {
            Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
                let account = Self::convert_account(pTradingAccount);
                self.subscription.send(&ServerEvent::AccountQuery(account));
            });
        }
    }

    fn on_rsp_qry_instrument(&mut self, pInstrument: *mut CThostFtdcInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, _bIsLast: bool) {
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
            let symbol_info = Self::convert_symbol_info(pInstrument);
            self.subscription.send(&ServerEvent::SymbolQuery(symbol_info));
        });
    }
}


fn _format_date(date_str: &str) -> String {
    let year = &date_str[0..4];
    let month = &date_str[4..6];
    let day = &date_str[6..8];
    format!("{}-{}-{}", year, month, day)
}