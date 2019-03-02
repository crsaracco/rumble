use std::net::TcpStream;
use std::io::{Write, Result, Read};
use native_tls::TlsConnector;
use native_tls::TlsConnectorBuilder;
use native_tls::TlsStream;

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
        let mut stream = connector.connect("localhost", tcp_stream).unwrap();

        Self {
            tls_stream: stream,
        }
    }
}

impl Write for Network {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.tls_stream.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.tls_stream.flush()
    }
}

impl Read for Network {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.tls_stream.read(buf)
    }
}