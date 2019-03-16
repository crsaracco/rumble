use native_tls::TlsConnector;
use native_tls::TlsStream;
use std::net::TcpStream;

use crate::message::MumbleMessage;

#[derive(Debug)]
pub enum NetworkError {
    ProstEncodeError(prost::EncodeError),
}

impl std::convert::From<prost::EncodeError> for NetworkError {
    fn from(prost_error: prost::EncodeError) -> NetworkError {
        NetworkError::ProstEncodeError(prost_error)
    }
}

pub struct Network {
    tls_stream: TlsStream<TcpStream>,
}

impl Network {
    pub fn new() -> Self {
        // TODO: Fix this. Insecure!
        let connector = TlsConnector::builder()
            .use_sni(false)
            .danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        // TODO: Accept connections to places other than localhost
        let tcp_stream = TcpStream::connect("localhost:64738").unwrap();
        let stream = connector.connect("localhost", tcp_stream).unwrap();

        Self {
            tls_stream: stream,
        }
    }

    pub fn send(&mut self, message: Box<dyn MumbleMessage>) -> Result<(), NetworkError> {
        let bytes = message.to_bytes()?;

        println!("{:?}", bytes);

        Ok(())
    }
}