/*
// Include the `mumble_proto` module, which is generated from mumble.proto.
pub mod mumble_proto {
    include!(concat!(env!("OUT_DIR"), "/mumble_proto.rs"));
}
*/

use super::message_id;
use super::mumble;
use prost::Message;

#[derive(Debug)]
pub enum ProtocolMessageError {
    ProstDecodeError(prost::DecodeError),
    ProstEncodeError(prost::EncodeError),
    InvalidMessage,
    NotEnoughBytes,
    TooManyBytes,
}

impl std::convert::From<prost::DecodeError> for ProtocolMessageError {
    fn from(prost_error: prost::DecodeError) -> ProtocolMessageError {
        ProtocolMessageError::ProstDecodeError(prost_error)
    }
}

impl std::convert::From<prost::EncodeError> for ProtocolMessageError {
    fn from(prost_error: prost::EncodeError) -> ProtocolMessageError {
        ProtocolMessageError::ProstEncodeError(prost_error)
    }
}

// TODO: This is non-ideal, try to make it better somehow?
pub fn bytes_to_message(bytes: Vec<u8>) -> Result<Box<dyn prost::Message>, ProtocolMessageError> {
    if bytes.len() < 6 {
        return Err(ProtocolMessageError::NotEnoughBytes);
    }

    let message_id: u32 = { ((bytes[0] as u32) << 8) + (bytes[1] as u32) };

    let message_len: u32 = {
        ((bytes[2] as u32) << 24)
            + ((bytes[3] as u32) << 16)
            + ((bytes[4] as u32) << 8)
            + (bytes[5] as u32)
    };

    if bytes.len() < (message_len + 6) as usize {
        return Err(ProtocolMessageError::NotEnoughBytes);
    } else if bytes.len() > (message_len + 6) as usize {
        return Err(ProtocolMessageError::TooManyBytes);
    }

    // TODO: This, in particular, I'd like to clean up.
    #[cfg_attr(rustfmt, rustfmt_skip)]
    match message_id {
        0 => Ok(Box::new(mumble::Version::decode(&bytes[6..])?) as Box<prost::Message>),
        1 => Ok(Box::new(mumble::UdpTunnel::decode(&bytes[6..])?) as Box<prost::Message>),
        2 => Ok(Box::new(mumble::Authenticate::decode(&bytes[6..])?) as Box<prost::Message>),
        3 => Ok(Box::new(mumble::Ping::decode(&bytes[6..])?) as Box<prost::Message>),
        4 => Ok(Box::new(mumble::Reject::decode(&bytes[6..])?) as Box<prost::Message>),
        5 => Ok(Box::new(mumble::ServerSync::decode(&bytes[6..])?) as Box<prost::Message>),
        6 => Ok(Box::new(mumble::ChannelRemove::decode(&bytes[6..])?) as Box<prost::Message>),
        7 => Ok(Box::new(mumble::ChannelState::decode(&bytes[6..])?) as Box<prost::Message>),
        8 => Ok(Box::new(mumble::UserRemove::decode(&bytes[6..])?) as Box<prost::Message>),
        9 => Ok(Box::new(mumble::UserState::decode(&bytes[6..])?) as Box<prost::Message>),
        10 => Ok(Box::new(mumble::BanList::decode(&bytes[6..])?) as Box<prost::Message>),
        11 => Ok(Box::new(mumble::TextMessage::decode(&bytes[6..])?) as Box<prost::Message>),
        12 => Ok(Box::new(mumble::PermissionDenied::decode(&bytes[6..])?) as Box<prost::Message>),
        13 => Ok(Box::new(mumble::Acl::decode(&bytes[6..])?) as Box<prost::Message>),
        14 => Ok(Box::new(mumble::QueryUsers::decode(&bytes[6..])?) as Box<prost::Message>),
        15 => Ok(Box::new(mumble::CryptSetup::decode(&bytes[6..])?) as Box<prost::Message>),
        16 => Ok(Box::new(mumble::ContextActionModify::decode(&bytes[6..])?) as Box<prost::Message>),
        17 => Ok(Box::new(mumble::ContextAction::decode(&bytes[6..])?) as Box<prost::Message>),
        18 => Ok(Box::new(mumble::UserList::decode(&bytes[6..])?) as Box<prost::Message>),
        19 => Ok(Box::new(mumble::VoiceTarget::decode(&bytes[6..])?) as Box<prost::Message>),
        20 => Ok(Box::new(mumble::PermissionQuery::decode(&bytes[6..])?) as Box<prost::Message>),
        21 => Ok(Box::new(mumble::CodecVersion::decode(&bytes[6..])?) as Box<prost::Message>),
        22 => Ok(Box::new(mumble::UserStats::decode(&bytes[6..])?) as Box<prost::Message>),
        23 => Ok(Box::new(mumble::RequestBlob::decode(&bytes[6..])?) as Box<prost::Message>),
        24 => Ok(Box::new(mumble::ServerConfig::decode(&bytes[6..])?) as Box<prost::Message>),
        25 => Ok(Box::new(mumble::SuggestConfig::decode(&bytes[6..])?) as Box<prost::Message>),
        _ => Err(ProtocolMessageError::InvalidMessage),
    }
}

pub fn message_to_bytes<T: prost::Message + message_id::MessageId>(
    message: T,
) -> Result<Vec<u8>, ProtocolMessageError> {
    let mut protobuf_payload = vec![];
    message.encode(&mut protobuf_payload)?;

    let mut output_buffer = vec![0x00, message.id()];
    output_buffer.append(&mut protobuf_payload_length_bytes(&protobuf_payload));
    output_buffer.append(&mut protobuf_payload);

    Ok(output_buffer)
}

fn protobuf_payload_length_bytes(protobuf_payload: &Vec<u8>) -> Vec<u8> {
    let mut length = vec![0u8; 4];
    length[0] = ((protobuf_payload.len() >> (8 * 3)) & 0xff) as u8;
    length[1] = ((protobuf_payload.len() >> (8 * 2)) & 0xff) as u8;
    length[2] = ((protobuf_payload.len() >> (8 * 1)) & 0xff) as u8;
    length[3] = ((protobuf_payload.len() >> (8 * 0)) & 0xff) as u8;

    length
}
