#[macro_use] extern crate prost_derive;

use std::time::{Instant, Duration};
use std::thread;

mod message;
mod network;
mod sasayaku_error;
mod logger;

use message::MumbleMessage;
use sasayaku_error::SasayakuError;

fn main() {
    println!("Proto location: {}", concat!(env!("OUT_DIR"), "/mumble_proto.rs"));

    let mut messages = vec![];

    // Version message
    let mut version_message = message::mumble::Version::default();
    version_message.version = Some(0x00010300);
    version_message.os = Some("X11".into());
    version_message.os_version = Some("Arch Linux".into());
    version_message.release = Some("Sasayaku 0.1.0".into());
    let m1 = MumbleMessage::Version(version_message);
    messages.push(m1);

    // Authenticate message
    let mut authenticate_message = message::mumble::Authenticate::default();
    authenticate_message.username = Some("testuser".into());
    let m2 = MumbleMessage::Authenticate(authenticate_message);
    messages.push(m2);

    let mut net = network::Network::new();
    for m in messages {
        net.send(m);
    }

    let mut ping_instant = Instant::now();

    loop {
        loop {
            let message = net.recv_one();

            match message {
                Ok(m) => logger::recv(&m),
                Err(SasayakuError::NotEnoughBytesToDecode) => {
                    //println!("{:?}", message);
                    break;
                }
                Err(e) => {
                    println!("{:?}", e);
                }
                _ => { }
            }
        }

        if ping_instant.elapsed() > Duration::new(10, 0) {
            let ping_message = message::mumble::Ping::default();
            let ping_message = MumbleMessage::Ping(ping_message);
            net.send(ping_message);
            ping_instant = Instant::now();
        }

        //println!("Done receiving messages. Sleeping for 0.25 seconds.");
        thread::sleep(Duration::new(0, 100000000))

    }

}