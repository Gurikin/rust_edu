use crate::error::{ConnectError, RecvError, RequestError};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

/// Соединение с клиентом.
/// Позволяет обрабатывать запросы.
pub struct UdpConnection {
    socket: UdpSocket,
}

impl UdpConnection {
    pub fn bind_port<Addrs>(local_addrs: Addrs) -> Result<Self, ConnectError>
    where
        Addrs: ToSocketAddrs,
    {
        let socket = UdpSocket::bind(local_addrs).map_err(|_| ConnectError::BadPortBound)?;
        socket.set_read_timeout(Some(Duration::from_secs(5)))?;
        Ok(Self { socket })
    }

    /// Обрабатываем запрос и возвращаем ответ используя логику
    /// предоставленную вызывающей стороной.
    pub fn process_request<Addrs>(&self, req: &str, peer_addr: Addrs) -> Result<usize, RequestError>
    where
        Addrs: ToSocketAddrs,
    {
        let response = self.socket.send_to(req.as_bytes(), peer_addr);
        match response {
            Ok(s) => Ok(s),
            Err(e) => {
                eprint!("Send error: {e}");
                Err(RequestError::Recv(RecvError::BadEncoding))
            }
        }
    }

    pub fn process_response(&self) -> Result<String, RequestError> {
        let response = super::recv_bytes(&self.socket);
        match response {
            Ok(s) => Ok(s),
            Err(_) => Err(RequestError::Recv(RecvError::BadEncoding)),
        }
    }

    /// Address of connected client
    pub fn peer_addr(&self) -> SocketAddr {
        match self.socket.peer_addr() {
            Ok(a) => a,
            Err(_) => SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 55331),
        }
    }
}
