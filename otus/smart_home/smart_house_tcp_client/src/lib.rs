use std::net::ToSocketAddrs;
use stp::client::StpClient;
use stp::error::{ConnectError, RequestError};

/// Клиент чата.
pub struct SmartHouseTcpClient {
    stp: StpClient,
}

impl SmartHouseTcpClient {
    /// Подключаемся к серверу.
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> Result<Self, ConnectError> {
        let stp = StpClient::connect(addr)?;
        Ok(Self { stp })
    }

    /// Переключаем состояние устройства.
    pub fn switch_state(&mut self, msg: &str) -> Result<String, RequestError> {
        let request = format!("switch_state:{}", msg);
        self.stp.send_request(request)
    }

    pub fn get_state(&mut self, msg: &str) -> Result<String, RequestError> {
        let request = format!("get_state:{}", msg);
        self.stp.send_request(request)
    }

    /// Добавляем комнату.
    pub fn add_room(&mut self, msg: &str) -> Result<String, RequestError> {
        let request = format!("add_room:{}", msg);
        self.stp.send_request(request)
    }
}
