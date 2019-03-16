//! Going to refactor this whole module into the new `network` module.
//! Don't look here.

/*
// Include the `mumble_proto` module, which is generated from mumble.proto.
pub mod mumble_proto {
    include!(concat!(env!("OUT_DIR"), "/mumble_proto.rs"));
}
*/

pub mod message_id;
pub mod mumble;
pub mod protocol_message;

use native_tls::TlsConnector;
use native_tls::TlsStream;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Network {
    tls_stream: TlsStream<TcpStream>,
    buffer: VecDeque<u8>,
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
            buffer: VecDeque::new(),
        }
    }

    pub fn send<T: prost::Message + message_id::MessageId>(
        &mut self,
        message: T,
    ) -> Result<(), protocol_message::ProtocolMessageError> {
        let bytes = protocol_message::message_to_bytes(message)?;

        // TODO: proper error handling (might combine ProtocolMessageError into "NetworkError"?)
        let write_result = self.tls_stream.write(&bytes);
        if write_result.is_err() {
            println!("Send: write error!");
        } else {
            println!("Send: write ok!");
        }

        // TODO: proper error handling (might combine ProtocolMessageError into "NetworkError"?)
        let flush_result = self.tls_stream.flush();
        if flush_result.is_err() {
            println!("Send: flush error!");
        } else {
            println!("Send: flush ok!");
        }

        Ok(())
    }

    // TODO: proper error handling
    pub fn recv(&mut self) -> Vec<Box<prost::Message>> {
        // Read bytes from the tls stream
        let mut buf = [0u8; 1024];
        let read_result = self.tls_stream.read(&mut buf);
        if read_result.is_err() {
            println!("Recv: read error!");
            return vec![];
        }

        // Read all bytes into buffer
        let bytes_len = read_result.unwrap();
        for i in 0..bytes_len {
            self.push_byte(buf[i]);
        }

        // Try to consume bytes into messages until we get None
        let mut messages = vec![];
        loop {
            let message = self.consume_bytes();
            if message.is_none() {
                return messages;
            } else {
                messages.push(message.unwrap());
            }
        }
    }

    // TODO: proper error handling
    fn consume_bytes(&mut self) -> Option<Box<prost::Message>> {
        // Check that we have enough bytes to make a message.
        let message_length = self.get_message_length_in_buffer()?;
        let required_buffer_length = (message_length + 6) as usize;
        if required_buffer_length > self.buffer.len() {
            return None;
        }

        // Consume the correct amount of bytes to create one packet
        let mut message_bytes = vec![];
        for _ in 0..required_buffer_length {
            message_bytes.push(self.buffer.pop_front().unwrap());
        }

        // Turn those bytes into a packet
        //println!("Message:\n{:?}", message_bytes);
        let message = protocol_message::bytes_to_message(message_bytes);
        if message.is_err() {
            println!("Consume: protocol message error: {:?}", message.err());
            return None;
        }
        Some(message.unwrap())
    }

    fn push_byte(&mut self, byte: u8) {
        self.buffer.push_back(byte);
    }

    fn get_message_length_in_buffer(&mut self) -> Option<u32> {
        if self.buffer.len() < 6 {
            return None;
        }

        let mut message_length = 0u32;
        message_length += (self.buffer[2] as u32) << (8 * 3);
        message_length += (self.buffer[3] as u32) << (8 * 2);
        message_length += (self.buffer[4] as u32) << (8 * 1);
        message_length += (self.buffer[5] as u32) << (8 * 0);
        Some(message_length)
    }
}