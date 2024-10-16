use crate::http::{request::Request, Method};

pub struct CountdownCancelAll {
    pub symbol: String,
    pub countdown_time: i64,
    pub recv_window: Option<i64>,
}

impl CountdownCancelAll {
    pub fn new(symbol: &str, countdown_time: i64) -> Self {
        Self {
            symbol: symbol.to_owned(),
            countdown_time,
            recv_window: None,
        }
    }

    pub fn set_recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("symbol".to_owned(), self.symbol.clone()));
        params.push(("countdownTime".to_owned(), self.countdown_time.to_string()));

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        params
    }
}


impl From<CountdownCancelAll> for Request {
    fn from(request: CountdownCancelAll) -> Request {
        let params = request.get_params();

        Request {
            path: "/fapi/v1/countdownCancelAll".to_owned(),
            method: Method::Post,
            params,
            credentials: None,
            sign: true,
        }
    }
}
