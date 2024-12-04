use stp::client::StpClient;
use stp::error::{ConnectError, RequestError};
use tokio::net::ToSocketAddrs;

/// Клиент чата.
pub struct SmartHouseTcpClient {
    stp: StpClient,
}

impl SmartHouseTcpClient {
    /// Подключаемся к серверу.
    pub async fn new<Addr: ToSocketAddrs>(addr: Addr) -> Result<Self, ConnectError> {
        let stp = StpClient::connect(addr).await?;
        Ok(Self { stp })
    }

    /// Переключаем состояние устройства.
    pub async fn switch_state(&mut self, msg: &str) -> Result<String, RequestError> {
        let request = format!("switch_state:{}", msg);
        self.stp.send_request(request).await
    }

    pub async fn get_state(&mut self, msg: &str) -> Result<String, RequestError> {
        let request = format!("get_state:{}", msg);
        self.stp.send_request(request).await
    }

    /// Добавляем комнату.
    pub async fn add_room(&mut self, msg: &str) -> Result<String, RequestError> {
        let request = format!("add_room:{}", msg);
        self.stp.send_request(request).await
    }

    /// Добавляем устройство
    pub async fn add_device(&mut self, msg: &str) -> Result<String, RequestError> {
        let request = format!("add_device:{}", msg);
        self.stp.send_request(request).await
    }

    /// Получаем отчет об умном доме
    pub async fn create_report(&mut self) -> Result<String, RequestError> {
        let request = "create_report".to_string();
        self.stp.send_request(request).await
    }
}
