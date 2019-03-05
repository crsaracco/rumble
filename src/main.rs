extern crate bytes;
extern crate native_tls;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate crossbeam;

mod network;

fn main() {
    /*
    println!("Proto location: {}", concat!(env!("OUT_DIR"), "/mumble_proto.rs"));
    */

    let mut net = network::Network::new();

    let mut version_message = network::mumble::Version::default();
    version_message.version = Some(0x00010300);
    version_message.release = Some("sasayaku 0.1.0".to_string());
    version_message.os = Some("X11".to_string());
    version_message.os_version = Some("Arch Linux".to_string());
    net.send(version_message).unwrap();

    let mut authenticate_message = network::mumble::Authenticate::default();
    authenticate_message.username = Some("testuser".to_string());
    net.send(authenticate_message).unwrap();

    loop {
        let messages = net.recv();
        for message in messages {
            println!("{:?}", message);
        }
    }
}
