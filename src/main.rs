extern crate bytes;
extern crate native_tls;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate crossbeam;

mod message;
mod network;

use message::MumbleMessage;

fn main() {
    /*
    println!("Proto location: {}", concat!(env!("OUT_DIR"), "/mumble_proto.rs"));
    */

    let mut net = network::Network::new();


    let mut messages: Vec<Box<MumbleMessage>> = vec![];

    let mut version_message = message::Version::default();
    version_message.version = Some(0x00010300);
    messages.push(Box::new(version_message));

    let mut authenticate_message = message::Authenticate::default();
    authenticate_message.username = Some("someuser".into());
    messages.push(Box::new(authenticate_message));

    let mut ping_message = message::Ping::default();
    messages.push(Box::new(ping_message));

    for message in messages {
        net.send(message);
    }

    // Ignore this stuff:
    /*
    let mut net = old_network::Network::new();

    let mut version_message = old_network::mumble::Version::default();
    version_message.version = Some(0x00010300);
    version_message.release = Some("sasayaku 0.1.0".to_string());
    version_message.os = Some("X11".to_string());
    version_message.os_version = Some("Arch Linux".to_string());
    net.send(version_message).unwrap();

    let mut authenticate_message = old_network::mumble::Authenticate::default();
    authenticate_message.username = Some("testuser".to_string());
    net.send(authenticate_message).unwrap();

    loop {
        let messages = net.recv();
        for message in messages {
            println!("{:?}", message);
        }
    }
    */
}