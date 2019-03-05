// TODO: Make this nicer somehow.

use super::mumble;

pub trait MessageId {
    fn id(&self) -> u8;
}

impl MessageId for mumble::Version {
    fn id(&self) -> u8 {
        0
    }
}

impl MessageId for mumble::UdpTunnel {
    fn id(&self) -> u8 {
        1
    }
}

impl MessageId for mumble::Authenticate {
    fn id(&self) -> u8 {
        2
    }
}

impl MessageId for mumble::Ping {
    fn id(&self) -> u8 {
        3
    }
}

impl MessageId for mumble::Reject {
    fn id(&self) -> u8 {
        4
    }
}

impl MessageId for mumble::ServerSync {
    fn id(&self) -> u8 {
        5
    }
}

impl MessageId for mumble::ChannelRemove {
    fn id(&self) -> u8 {
        6
    }
}

impl MessageId for mumble::ChannelState {
    fn id(&self) -> u8 {
        7
    }
}

impl MessageId for mumble::UserRemove {
    fn id(&self) -> u8 {
        8
    }
}

impl MessageId for mumble::UserState {
    fn id(&self) -> u8 {
        9
    }
}

impl MessageId for mumble::BanList {
    fn id(&self) -> u8 {
        10
    }
}

impl MessageId for mumble::TextMessage {
    fn id(&self) -> u8 {
        11
    }
}

impl MessageId for mumble::PermissionDenied {
    fn id(&self) -> u8 {
        12
    }
}

impl MessageId for mumble::Acl {
    fn id(&self) -> u8 {
        13
    }
}

impl MessageId for mumble::QueryUsers {
    fn id(&self) -> u8 {
        14
    }
}

impl MessageId for mumble::CryptSetup {
    fn id(&self) -> u8 {
        15
    }
}

impl MessageId for mumble::ContextActionModify {
    fn id(&self) -> u8 {
        16
    }
}

impl MessageId for mumble::ContextAction {
    fn id(&self) -> u8 {
        17
    }
}

impl MessageId for mumble::UserList {
    fn id(&self) -> u8 {
        18
    }
}

impl MessageId for mumble::VoiceTarget {
    fn id(&self) -> u8 {
        19
    }
}

impl MessageId for mumble::PermissionQuery {
    fn id(&self) -> u8 {
        20
    }
}

impl MessageId for mumble::CodecVersion {
    fn id(&self) -> u8 {
        21
    }
}

impl MessageId for mumble::UserStats {
    fn id(&self) -> u8 {
        22
    }
}

impl MessageId for mumble::RequestBlob {
    fn id(&self) -> u8 {
        23
    }
}

impl MessageId for mumble::ServerConfig {
    fn id(&self) -> u8 {
        24
    }
}

impl MessageId for mumble::SuggestConfig {
    fn id(&self) -> u8 {
        25
    }
}
