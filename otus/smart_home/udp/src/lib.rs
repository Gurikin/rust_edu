// use crate::error::{RecvError, SendError};
use std::{
    // fmt::Error,
    // io::{Read, Write},
    net::UdpSocket,
};

// pub mod client;
pub mod connection;
pub mod error;

/// Читает четыре байта длины, а потом сами данные.
fn recv_bytes(socket: &UdpSocket) -> Result<String, String> {
    let mut buf = [0; 1000];
    let number_of_bytes = socket.recv(&mut buf).expect("Didn't receive data");
    let result_buf = &mut buf[..number_of_bytes];
    match String::from_utf8(result_buf.to_vec()) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}
