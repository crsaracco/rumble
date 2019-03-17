use super::mumble;
use prost::Message;
use crate::rumble_error::RumbleError;

#[allow(dead_code)]
#[derive(Debug)]
pub enum MumbleMessage {
    Version(mumble::Version),
    UdpTunnel,
    Authenticate(mumble::Authenticate),
    Ping(mumble::Ping),
    Reject(mumble::Reject),
    ServerSync(mumble::ServerSync),
    ChannelRemove(mumble::ChannelRemove),
    ChannelState(mumble::ChannelState),
    UserRemove(mumble::UserRemove),
    UserState(mumble::UserState),
    BanList(mumble::BanList),
    TextMessage(mumble::TextMessage),
    PermissionDenied(mumble::PermissionDenied),
    Acl(mumble::Acl),
    QueryUsers(mumble::QueryUsers),
    CryptSetup(mumble::CryptSetup),
    ContextActionModify(mumble::ContextActionModify),
    ContextAction(mumble::ContextAction),
    UserList(mumble::UserList),
    VoiceTarget(mumble::VoiceTarget),
    PermissionQuery(mumble::PermissionQuery),
    CodecVersion(mumble::CodecVersion),
    UserStats(mumble::UserStats),
    RequestBlob(mumble::RequestBlob),
    ServerConfig(mumble::ServerConfig),
    SuggestConfig(mumble::SuggestConfig),
}

pub fn to_bytes(message: MumbleMessage) -> Vec<u8> {
    match message {
        MumbleMessage::Version(m) => encode(m, 0),
        MumbleMessage::UdpTunnel => vec![0, 1, 0, 0, 0, 0],
        MumbleMessage::Authenticate(m) => encode(m, 2),
        MumbleMessage::Ping(m) => encode(m, 3),
        MumbleMessage::Reject(m) => encode(m, 4),
        MumbleMessage::ServerSync(m) => encode(m, 5),
        MumbleMessage::ChannelRemove(m) => encode(m, 6),
        MumbleMessage::ChannelState(m) => encode(m, 7),
        MumbleMessage::UserRemove(m) => encode(m, 8),
        MumbleMessage::UserState(m) => encode(m, 9),
        MumbleMessage::BanList(m) => encode(m, 10),
        MumbleMessage::TextMessage(m) => encode(m, 11),
        MumbleMessage::PermissionDenied(m) => encode(m, 12),
        MumbleMessage::Acl(m) => encode(m, 13),
        MumbleMessage::QueryUsers(m) => encode(m, 14),
        MumbleMessage::CryptSetup(m) => encode(m, 15),
        MumbleMessage::ContextActionModify(m) => encode(m, 16),
        MumbleMessage::ContextAction(m) => encode(m, 17),
        MumbleMessage::UserList(m) => encode(m, 18),
        MumbleMessage::VoiceTarget(m) => encode(m, 19),
        MumbleMessage::PermissionQuery(m) => encode(m, 20),
        MumbleMessage::CodecVersion(m) => encode(m, 21),
        MumbleMessage::UserStats(m) => encode(m, 22),
        MumbleMessage::RequestBlob(m) => encode(m, 23),
        MumbleMessage::ServerConfig(m) => encode(m, 24),
        MumbleMessage::SuggestConfig(m) => encode(m, 25),
    }
}

pub fn from_bytes(bytes: Vec<u8>) -> Result<MumbleMessage, RumbleError> {
    let message_id = bytes[1];
    let payload = &bytes[6..];
    match message_id {
        0 => Ok(MumbleMessage::Version(mumble::Version::decode(payload)?)),
        1 => Ok(MumbleMessage::UdpTunnel),
        2 => Ok(MumbleMessage::Authenticate(mumble::Authenticate::decode(payload)?)),
        3 => Ok(MumbleMessage::Ping(mumble::Ping::decode(payload)?)),
        4 => Ok(MumbleMessage::Reject(mumble::Reject::decode(payload)?)),
        5 => Ok(MumbleMessage::ServerSync(mumble::ServerSync::decode(payload)?)),
        6 => Ok(MumbleMessage::ChannelRemove(mumble::ChannelRemove::decode(payload)?)),
        7 => Ok(MumbleMessage::ChannelState(mumble::ChannelState::decode(payload)?)),
        8 => Ok(MumbleMessage::UserRemove(mumble::UserRemove::decode(payload)?)),
        9 => Ok(MumbleMessage::UserState(mumble::UserState::decode(payload)?)),
        10 => Ok(MumbleMessage::BanList(mumble::BanList::decode(payload)?)),
        11 => Ok(MumbleMessage::TextMessage(mumble::TextMessage::decode(payload)?)),
        12 => Ok(MumbleMessage::PermissionDenied(mumble::PermissionDenied::decode(payload)?)),
        13 => Ok(MumbleMessage::Acl(mumble::Acl::decode(payload)?)),
        14 => Ok(MumbleMessage::QueryUsers(mumble::QueryUsers::decode(payload)?)),
        15 => Ok(MumbleMessage::CryptSetup(mumble::CryptSetup::decode(payload)?)),
        16 => Ok(MumbleMessage::ContextActionModify(mumble::ContextActionModify::decode(payload)?)),
        17 => Ok(MumbleMessage::ContextAction(mumble::ContextAction::decode(payload)?)),
        18 => Ok(MumbleMessage::UserList(mumble::UserList::decode(payload)?)),
        19 => Ok(MumbleMessage::VoiceTarget(mumble::VoiceTarget::decode(payload)?)),
        20 => Ok(MumbleMessage::PermissionQuery(mumble::PermissionQuery::decode(payload)?)),
        21 => Ok(MumbleMessage::CodecVersion(mumble::CodecVersion::decode(payload)?)),
        22 => Ok(MumbleMessage::UserStats(mumble::UserStats::decode(payload)?)),
        23 => Ok(MumbleMessage::RequestBlob(mumble::RequestBlob::decode(payload)?)),
        24 => Ok(MumbleMessage::ServerConfig(mumble::ServerConfig::decode(payload)?)),
        25 => Ok(MumbleMessage::SuggestConfig(mumble::SuggestConfig::decode(payload)?)),
        _ => Err(RumbleError::InvalidMessageId),
    }
}

fn encode<T: prost::Message>(mumble_message: T, id: u8) -> Vec<u8>{
    let mut protobuf_payload = vec![];
    mumble_message.encode(&mut protobuf_payload);

    let mut output_buffer = vec![0x00, id];
    output_buffer.append(&mut protobuf_payload_length_bytes(&protobuf_payload));
    output_buffer.append(&mut protobuf_payload);

    output_buffer
}

fn protobuf_payload_length_bytes(protobuf_payload: &Vec<u8>) -> Vec<u8> {
    let mut length = vec![0u8; 4];
    length[0] = ((protobuf_payload.len() >> (8 * 3)) & 0xff) as u8;
    length[1] = ((protobuf_payload.len() >> (8 * 2)) & 0xff) as u8;
    length[2] = ((protobuf_payload.len() >> (8 * 1)) & 0xff) as u8;
    length[3] = ((protobuf_payload.len() >> (8 * 0)) & 0xff) as u8;

    length
}