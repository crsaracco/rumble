extern crate native_tls;
extern crate bytes;
extern crate prost;
#[macro_use] extern crate prost_derive;

use std::io::{Read, Write};
use prost::Message;

mod network;

// Include the `mumble_proto` module, which is generated from mumble.proto.
pub mod mumble_proto {
    include!(concat!(env!("OUT_DIR"), "/mumble_proto.rs"));
}

fn protobuf_payload_length_bytes(protobuf_payload: &Vec<u8>) -> Vec<u8> {
    let mut length = vec![0u8; 4];
    length[0] = ((protobuf_payload.len() >> (8*3)) & 0xff) as u8;
    length[1] = ((protobuf_payload.len() >> (8*2)) & 0xff) as u8;
    length[2] = ((protobuf_payload.len() >> (8*1)) & 0xff) as u8;
    length[3] = ((protobuf_payload.len() >> (8*0)) & 0xff) as u8;

    length
}

macro_rules! prost_message_to_mumble_packet {
    ( $message_id:expr, $message:expr ) => {
        {
            let mut protobuf_payload = vec![];
            $message.encode(&mut protobuf_payload).unwrap();

            let mut output_buffer = vec![0x00, $message_id];
            output_buffer.append(&mut protobuf_payload_length_bytes(&protobuf_payload));
            output_buffer.append(&mut protobuf_payload);

            output_buffer
        }
    };
}

fn main() {
    println!("Proto location: {}", concat!(env!("OUT_DIR"), "/mumble_proto.rs"));

    let mut net = network::Network::new();

    let mut version_message = mumble_proto::Version::default();
    version_message.version = Some(0x00010300);
    version_message.release = Some("1.3.0".to_string());
    version_message.os = Some("Linux".to_string());
    version_message.os_version = Some("1.3.0".to_string());

    let packet = prost_message_to_mumble_packet!(0x00, &version_message);

    net.write_all(&packet).unwrap();

    let mut authenticate_message = mumble_proto::Authenticate::default();
    authenticate_message.username = Some("testuser".to_string());

    let packet = prost_message_to_mumble_packet!(0x02, &authenticate_message);
    net.write_all(&packet).unwrap();



    loop {
        let mut res = vec![];
        net.read(&mut res).unwrap();
        if res.len() > 0 {
            println!("{}", String::from_utf8_lossy(&res));
        }
    }
}