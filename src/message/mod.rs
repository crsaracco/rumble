use std::any::Any;

pub mod mumble;

/*
#[derive(Debug)]
pub enum Message {
    Version{ m: mumble::Version },
    UdpTunnel{ m: mumble::UdpTunnel },
    Authenticate{ m: mumble::Authenticate },
    Ping{ m: mumble::Ping },
    Reject{ m: mumble::Reject },
    ServerSync{ m: mumble::ServerSync },
    ChannelRemove{ m: mumble::ChannelRemove },
    ChannelState{ m: mumble::ChannelState },
    UserRemove{ m: mumble::UserRemove },
    UserState{ m: mumble::UserState },
    BanList{ m: mumble::BanList },
    TextMessage{ m: mumble::TextMessage },
    PermissionDenied{ m: mumble::PermissionDenied },
    Acl{ m: mumble::Acl },
    QueryUsers{ m: mumble::QueryUsers },
    CryptSetup{ m: mumble::CryptSetup },
    ContextActionModify{ m: mumble::ContextActionModify },
    ContextAction{ m: mumble::ContextAction },
    UserList{ m: mumble::UserList },
    VoiceTarget{ m: mumble::VoiceTarget },
    PermissionQuery{ m: mumble::PermissionQuery },
    CodecVersion{ m: mumble::CodecVersion },
    UserStats{ m: mumble::UserStats },
    RequestBlob{ m: mumble::RequestBlob },
    ServerConfig{ m: mumble::ServerConfig },
    SuggestConfig{ m: mumble::SuggestConfig },
}

impl Message {
    pub fn new_version() -> Self {
        Message::Version{ m: mumble::Version::default() }
    }
}
*/

pub trait MumbleMessage {
    fn as_any(&self) -> &dyn Any;
}

impl MumbleMessage for mumble::Version {
    fn as_any(&self) -> &dyn Any {
        self
    }
}