use std::net::TcpStream;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use native_tls::TlsConnector;
use native_tls::TlsStream;

use crate::message::{self, MumbleMessage};
use crate::sasayaku_error::SasayakuError;
use crate::logger;

pub struct Network {
    tls_stream: Arc<Mutex<TlsStream<TcpStream>>>,
    buffer: Arc<Mutex<VecDeque<u8>>>,
}

// TODO: Accept connections to places other than localhost
const SERVER_ADDRESS: &str = "localhost:64738";
const TLS_DOMAIN: &str = "localhost";

impl Network {
    pub fn new() -> Self {
        // TODO: Fix this. Insecure!
        let connector = TlsConnector::builder()
            .use_sni(false)
            .danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let tcp_stream = TcpStream::connect(SERVER_ADDRESS).unwrap();

        // Keep trying to connect to the tcp_stream until success.
        let mut stream = connector.connect(TLS_DOMAIN, tcp_stream).unwrap();
        stream.get_ref().set_nonblocking(true);
        let stream = Arc::new(Mutex::new(stream));
        let thread_stream = stream.clone();

        let buffer = Arc::new(Mutex::new(VecDeque::new()));
        let thread_buffer = buffer.clone();

        thread::spawn(|| {
            recv_bytes(thread_stream, thread_buffer);
        });

        Self {
            tls_stream: stream,
            buffer,
        }
    }

    pub fn send(&mut self, mumble_message: MumbleMessage) -> Result<(), SasayakuError> {
        logger::send(&mumble_message);
        let bytes = message::to_bytes(mumble_message);
        let mut tls_stream = self.tls_stream.lock();
        let mut tls_stream = tls_stream.as_mut().unwrap();

        tls_stream.write(&bytes)?;
        tls_stream.flush()?;

        Ok(())
    }

    pub fn recv_one(&mut self) -> Result<MumbleMessage, SasayakuError> {
        let mut buffer = self.buffer.lock();
        let buffer = buffer.as_mut().unwrap();

        let message_length = get_message_length_in_buffer(buffer)?;
        if buffer.len() >= (message_length + 6) {
            // Take message out of the VecDeque and decode it
            let mut message_bytes = vec![];
            for _ in 0..(message_length + 6) {
                message_bytes.push(buffer.pop_front().unwrap());
            }

            return Ok(message::from_bytes(message_bytes)?);
        }

        Err(SasayakuError::NotEnoughBytesToDecode)
    }
}

fn get_message_length_in_buffer(buffer: &MutexGuard<VecDeque<u8>>) -> Result<usize, SasayakuError> {
    if buffer.len() < 6 {
        return Err(SasayakuError::NotEnoughBytesToDecode);
    }

    let mut message_length = 0usize;
    message_length += (buffer[2] as usize) << (8 * 3);
    message_length += (buffer[3] as usize) << (8 * 2);
    message_length += (buffer[4] as usize) << (8 * 1);
    message_length += (buffer[5] as usize) << (8 * 0);
    Ok(message_length)
}

fn recv_bytes(tls_stream: Arc<Mutex<TlsStream<TcpStream>>>, buffer: Arc<Mutex<VecDeque<u8>>>) {
    let mut buf = [0u8; 1024];
    // Read bytes from the tls stream
    loop {
        let read_result = tls_stream.lock().as_mut().unwrap().read(&mut buf);
        if read_result.is_err() {
            thread::sleep(Duration::new(0, 100_000_000));
            continue;
        }

        // Read all bytes into buffer
        let num_bytes = read_result.unwrap();
        for i in 0..num_bytes {
            buffer.lock().unwrap().push_back(buf[i]);
        }
    }
}