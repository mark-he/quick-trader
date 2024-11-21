#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use libctp_sys::*;
use log::info;
use std::collections::HashMap;
use std::os::raw::*;
use common::{c::*, msmc::Subscription};
use crate::model::{Account, Order, Position, Session, SymbolInfo, Trade, TradeEvent};

use super::ctp_code::*;

pub struct Spi {
    pub subscription: Subscription<TradeEvent>,
    pub order_queue : HashMap<i32, Vec<Order>>,
    pub position_queue : HashMap<i32, Vec<Position>>,
}

impl Spi {
    pub fn new() -> Self {
        Spi { subscription: Subscription::<TradeEvent>::top(), order_queue: HashMap::new(), position_queue: HashMap::new() }
    }

    fn handle_result<F>(subscription: &Subscription<TradeEvent>, request_id: i32, pRspInfo: *mut CThostFtdcRspInfoField, f: &mut F)
        where F: FnMut() {      
        if !pRspInfo.is_null() {
            let pRspInfo = unsafe { &mut *pRspInfo };
            if pRspInfo.ErrorID != 0 as c_int {
                subscription.send(&TradeEvent::Error(request_id,  c_char_to_gbk_string(pRspInfo.ErrorMsg.as_ptr())));
                return
            }
        }
        f();
    }

    fn convert_order(pRspInfo: *mut CThostFtdcOrderField) -> Order {
        let pRspInfo = unsafe { &mut *pRspInfo };

        let offset = c_char_to_string(pRspInfo.CombOffsetFlag.as_ptr());
        let submit_status = (pRspInfo.OrderSubmitStatus as u8 as char).to_string();
        let status = (pRspInfo.OrderStatus as u8 as char).to_string();
        let direction = (pRspInfo.Direction as u8 as char).to_string();

        let order_type = OrderType {
            price_type: (pRspInfo.OrderPriceType as u8) as char,
            time_condition: (pRspInfo.TimeCondition as u8) as char,
            volume_condition: (pRspInfo.VolumeCondition as u8) as char,
        };
        let order = Order {
            order_ref: c_char_to_string(pRspInfo.OrderRef.as_ptr()),
            direction:  DIRECTION_REV.as_ref().get(&direction).unwrap().to_string(),
            offset: OFFSET_REV.as_ref().get(&offset).unwrap().to_string(),
            price: pRspInfo.LimitPrice,
            order_type: ORDER_TYPE_REV.as_ref().get(&order_type.to_string()).unwrap().to_string(),
            volume_total_original: pRspInfo.VolumeTotalOriginal as u32,   
            submit_status: ORDER_SUBMIT_REV.as_ref().get(&submit_status).unwrap().to_string(),
            sys_id: c_char_to_string(pRspInfo.OrderSysID.as_ptr()),
            status: ORDER_STATUS_REV.as_ref().get(&status).unwrap().to_string(),
            volume_traded: pRspInfo.VolumeTraded as u32,
            volume_total: pRspInfo.VolumeTotal as u32,
            status_msg: c_char_to_gbk_string(pRspInfo.StatusMsg.as_ptr()),
            symbol: c_char_to_string(pRspInfo.InstrumentID.as_ptr()),
            request_id: pRspInfo.RequestID,
            invest_unit_id: c_char_to_string(pRspInfo.InvestUnitID.as_ptr()),
        };
        order
    }

    fn convert_trade(pRspInfo: *mut CThostFtdcTradeField) -> Trade {
        let pRspInfo = unsafe { &mut *pRspInfo };

        let direction = (pRspInfo.Direction as u8 as char).to_string();
        let offset = (pRspInfo.OffsetFlag as u8 as char).to_string();
        let date = c_char_to_string(pRspInfo.TradeDate.as_ptr());
        let time = c_char_to_string(pRspInfo.TradeTime.as_ptr());
        let trade = Trade {
            order_ref: c_char_to_string(pRspInfo.OrderRef.as_ptr()),
            trade_id: c_char_to_string(pRspInfo.TradeID.as_ptr()),
            sys_id: c_char_to_string(pRspInfo.OrderSysID.as_ptr()),
            direction: DIRECTION_REV.as_ref().get(&direction).unwrap().to_string(),
            offset: OFFSET_REV.as_ref().get(&offset).unwrap().to_string(),
            price: pRspInfo.Price,
            volume: pRspInfo.Volume as u32,
            datetime: format!("{} {}", date, time),
            symbol: c_char_to_string(pRspInfo.InstrumentID.as_ptr()),
        };
        trade
    }

    fn convert_position(pRspInfo: *mut CThostFtdcInvestorPositionField) -> Position {
        let pRspInfo = unsafe { &mut *pRspInfo };
        info!("POSITION: {:?}", pRspInfo);
        let direction = (pRspInfo.PosiDirection as u8 as char).to_string();
        let position = Position {
            symbol : c_char_to_string(pRspInfo.InstrumentID.as_ptr()),
            position: pRspInfo.Position as u32,
            today_position: pRspInfo.TodayPosition as u32,
            direction: POSITION_DIRECTION_REV.as_ref().get(&direction).unwrap().to_string(),
            cost: pRspInfo.PositionCost,
            invest_unit_id: c_char_to_string(pRspInfo.InvestUnitID.as_ptr()),
        };
        position
    }

    fn convert_account(pRspInfo: *mut CThostFtdcTradingAccountField) -> Account {
        let pRspInfo = unsafe { &mut *pRspInfo };

        let account = Account {
            account_id : c_char_to_string(pRspInfo.AccountID.as_ptr()),
            interest: pRspInfo.Interest,
            balance: pRspInfo.Balance,
            available: pRspInfo.Available,
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
        self.subscription.send(&TradeEvent::Connected);
    }

    fn on_front_disconnected(&mut self, nReason: ::std::os::raw::c_int) {
        self.subscription.send(&TradeEvent::Disconnected(nReason)); 
    }

    fn on_heart_beat_warning(&mut self, nTimeLapse: ::std::os::raw::c_int) {
        self.subscription.send(&TradeEvent::HeartBeatWarning(nTimeLapse)); 
    }

    fn on_rsp_user_login(&mut self, _pRspUserLogin: *mut CThostFtdcRspUserLoginField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, _bIsLast: bool) { 
        //let pRspUserLogin = unsafe { &mut *pRspUserLogin };
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
            let session = Self::convert_session(_pRspUserLogin);
            self.subscription.send(&TradeEvent::UserLogin(session));
        });
    }

    fn on_rsp_user_logout(&mut self, _pUserLogout: *mut CThostFtdcUserLogoutField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, _bIsLast: bool) { 
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
            self.subscription.send(&TradeEvent::UserLogout);
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
            self.subscription.send(&TradeEvent::SettlementConfirmed);
        });
    }

    fn on_rtn_order(&mut self, pOrder: *mut CThostFtdcOrderField) {
        let ret = Self::convert_order(pOrder);
        let _ = self.subscription.send(&TradeEvent::OnOrder(ret));
    }

    fn on_rtn_trade(&mut self, pTrade: *mut CThostFtdcTradeField) {
        let ret = Self::convert_trade(pTrade);
        let _ = self.subscription.send(&TradeEvent::OnTrade(ret));
    }

    fn on_rsp_qry_order(&mut self, pOrder: *mut CThostFtdcOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) { 
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
            let order = Self::convert_order(pOrder);
            let request_id = nRequestID as i32;
            self.order_queue.entry(request_id).or_insert_with(Vec::new).push(order);
            if bIsLast {
                let mut ret = self.order_queue.remove(&request_id);
                if let Some(order_vec) = ret.take() {
                    let _ = self.subscription.send(&TradeEvent::OrderQuery(order_vec));
                }
            }
        });
    }

    fn on_rsp_qry_investor_position(&mut self, pInvestorPosition: *mut CThostFtdcInvestorPositionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) { 
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
            let position = Self::convert_position(pInvestorPosition);
            self.position_queue.entry(nRequestID).or_insert_with(Vec::new).push(position);
            if bIsLast {
                let mut ret = self.position_queue.remove(&nRequestID);
                if let Some(position_vec) = ret.take() {
                    self.subscription.send(&TradeEvent::PositionQuery(position_vec));
                }
            }
        });
    }

    fn on_rsp_qry_trading_account(&mut self, pTradingAccount: *mut CThostFtdcTradingAccountField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) { 
        if bIsLast {
            Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
                let account = Self::convert_account(pTradingAccount);
                self.subscription.send(&TradeEvent::AccountQuery(account));
            });
        }
    }

    fn on_rsp_qry_instrument(&mut self, pInstrument: *mut CThostFtdcInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, _bIsLast: bool) {
        Self::handle_result(&self.subscription, nRequestID, pRspInfo, &mut ||{
            let symbol_info = Self::convert_symbol_info(pInstrument);
            self.subscription.send(&TradeEvent::SymbolQuery(symbol_info));
        });
    }
}
