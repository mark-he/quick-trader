use crate::websocket::Stream;
use std::io::{Read, Write};
use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Error, Message, WebSocket};
pub struct BybitWebSocketClient;

impl BybitWebSocketClient {
    pub fn connect_with_url(url: &str) -> Result<WebSocketState<MaybeTlsStream<TcpStream>>, Error> {
        let (socket, response) = connect(url)?;

        log::info!("Connected to {}", url);
        log::debug!("Response HTTP code: {}", response.status());
        log::debug!("Response headers:");
        for (ref header, _value) in response.headers() {
            log::debug!("* {}", header);
        }

        Ok(WebSocketState::new(socket))
    }

    pub fn connect() -> Result<WebSocketState<MaybeTlsStream<TcpStream>>, Error> {
        BybitWebSocketClient::connect_with_url(&crate::config::wss_api())
    }
}

pub struct WebSocketState<T> {
    socket: WebSocket<T>,
    id: u64,
}

impl<T: Read + Write> WebSocketState<T> {
    pub fn new(socket: WebSocket<T>) -> Self {
        Self { socket, id: 0 }
    }

    fn send<'a>(&mut self, method: &str, params: impl IntoIterator<Item = &'a str>) -> u64 {
        let mut params_str: String = params
            .into_iter()
            .map(|param| format!("\"{}\"", param))
            .collect::<Vec<String>>()
            .join(",");

        if !params_str.is_empty() {
            params_str = format!("\"params\": [{params}],", params = params_str)
        };

        let s = format!(
            "{{\"op\":\"{method}\",{params}\"id\":{id}}}",
            method = method,
            params = params_str,
            id = self.id
        );
        let message = Message::Text(s);
        log::debug!("Sent {}", message);

        self.socket.send(message).unwrap();

        self.id += 1;
        self.id
    }

    pub fn subscribe<'a>(&mut self, streams: impl IntoIterator<Item = &'a Stream>) -> u64 {
        self.send("subscribe", streams.into_iter().map(|s| s.as_str()))
    }

    pub fn subscribe_from_slice(&mut self, streams: &[Stream]) -> u64 {
        self.send("subscribe", streams.iter().map(|s| s.as_str()))
    }

    pub fn unsubscribe<'a>(&mut self, streams: impl IntoIterator<Item = &'a Stream>) -> u64 {
        self.send("unsubscribe", streams.into_iter().map(|s| s.as_str()))
    }


    pub fn close(&mut self) -> Result<(), Error> {
        self.socket.close(None)
    }
}

impl<T> From<WebSocketState<T>> for WebSocket<T> {
    fn from(conn: WebSocketState<T>) -> WebSocket<T> {
        conn.socket
    }
}

impl<T> AsMut<WebSocket<T>> for WebSocketState<T> {
    fn as_mut(&mut self) -> &mut WebSocket<T> {
        &mut self.socket
    }
}
