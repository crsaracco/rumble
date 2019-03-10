use super::*;
use std::any::Any;

fn protobuf_payload_length_bytes(protobuf_payload: &Vec<u8>) -> Vec<u8> {
    let mut length = vec![0u8; 4];
    length[0] = ((protobuf_payload.len() >> (8 * 3)) & 0xff) as u8;
    length[1] = ((protobuf_payload.len() >> (8 * 2)) & 0xff) as u8;
    length[2] = ((protobuf_payload.len() >> (8 * 1)) & 0xff) as u8;
    length[3] = ((protobuf_payload.len() >> (8 * 0)) & 0xff) as u8;

    length
}


pub trait MumbleMessage: prost::Message {
    fn id(&self) -> u8;

    // Unfortunately this can't have a default implementation;
    // it has to be implemented for each type separately.
    // If you try to define it here, you get:
    // "the `as_any` method cannot be invoked on a trait object"
    // TODO: fix this?
    fn as_any(&self) -> &dyn Any;

    // Unfortunately this can't have a default implementation;
    // it has to be implemented for each type separately.
    // If you try to define it here, you get:
    // "the `to_bytes` method cannot be invoked on a trait object"
    // TODO: fix this?
    fn to_bytes(&self) -> Result<Vec<u8>, prost::EncodeError>
    {
        let mut protobuf_payload = vec![];
        self.encode(&mut protobuf_payload)?;

        let mut output_buffer = vec![0x00, self.id()];
        output_buffer.append(&mut protobuf_payload_length_bytes(&protobuf_payload));
        output_buffer.append(&mut protobuf_payload);

        Ok(output_buffer)
    }
}

// Bunch of `impl`s. Pretty hacky, but it works.

// Message ID 0
impl MumbleMessage for mumble::Version {
    fn id(&self) -> u8 {
        0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 1
impl MumbleMessage for mumble::UdpTunnel {
    fn id(&self) -> u8 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 2
impl MumbleMessage for mumble::Authenticate {
    fn id(&self) -> u8 {
        2
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 3
impl MumbleMessage for mumble::Ping {
    fn id(&self) -> u8 {
        3
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 4
impl MumbleMessage for mumble::Reject {
    fn id(&self) -> u8 {
        4
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 5
impl MumbleMessage for mumble::ServerSync {
    fn id(&self) -> u8 {
        5
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 6
impl MumbleMessage for mumble::ChannelRemove {
    fn id(&self) -> u8 {
        6
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 7
impl MumbleMessage for mumble::ChannelState {
    fn id(&self) -> u8 {
        7
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 8
impl MumbleMessage for mumble::UserRemove {
    fn id(&self) -> u8 {
        8
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 9
impl MumbleMessage for mumble::UserState {
    fn id(&self) -> u8 {
        9
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 10
impl MumbleMessage for mumble::BanList {
    fn id(&self) -> u8 {
        10
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 11
impl MumbleMessage for mumble::TextMessage {
    fn id(&self) -> u8 {
        11
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 12
impl MumbleMessage for mumble::PermissionDenied {
    fn id(&self) -> u8 {
        12
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 13
impl MumbleMessage for mumble::Acl {
    fn id(&self) -> u8 {
        13
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 14
impl MumbleMessage for mumble::QueryUsers {
    fn id(&self) -> u8 {
        14
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 15
impl MumbleMessage for mumble::CryptSetup {
    fn id(&self) -> u8 {
        15
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 16
impl MumbleMessage for mumble::ContextActionModify {
    fn id(&self) -> u8 {
        16
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 17
impl MumbleMessage for mumble::ContextAction {
    fn id(&self) -> u8 {
        17
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 18
impl MumbleMessage for mumble::UserList {
    fn id(&self) -> u8 {
        18
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 19
impl MumbleMessage for mumble::VoiceTarget {
    fn id(&self) -> u8 {
        19
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 20
impl MumbleMessage for mumble::PermissionQuery {
    fn id(&self) -> u8 {
        20
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 21
impl MumbleMessage for mumble::CodecVersion {
    fn id(&self) -> u8 {
        21
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 22
impl MumbleMessage for mumble::UserStats {
    fn id(&self) -> u8 {
        22
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 23
impl MumbleMessage for mumble::RequestBlob {
    fn id(&self) -> u8 {
        23
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 24
impl MumbleMessage for mumble::ServerConfig {
    fn id(&self) -> u8 {
        24
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Message ID 25
impl MumbleMessage for mumble::SuggestConfig {
    fn id(&self) -> u8 {
        25
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
