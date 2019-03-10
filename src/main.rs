extern crate bytes;
extern crate native_tls;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate crossbeam;

mod message;
mod network;
mod new_network;

use message::MumbleMessage;

fn main() {
    /*
    println!("Proto location: {}", concat!(env!("OUT_DIR"), "/mumble_proto.rs"));
    */

    let mut net = new_network::NewNetwork::new();


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

    /*
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
    */
}

fn handle_mumble_message(message: Box<dyn MumbleMessage>) {
    if let Some(m) = message.as_any().downcast_ref::<message::Version>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::UdpTunnel>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::Authenticate>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::Ping>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::Reject>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::ServerSync>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::ChannelRemove>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::ChannelState>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::UserRemove>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::UserState>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::BanList>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::TextMessage>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::PermissionDenied>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::Acl>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::QueryUsers>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::CryptSetup>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::ContextActionModify>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::ContextAction>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::UserList>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::VoiceTarget>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::PermissionQuery>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::CodecVersion>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::UserStats>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::RequestBlob>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::ServerConfig>() {
        // something
    } else if let Some(m) = message.as_any().downcast_ref::<message::SuggestConfig>() {
        // something
    }
}