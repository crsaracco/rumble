use colored::*;
use crate::message::MumbleMessage;

pub fn send(message: &MumbleMessage) {
    println!("{}", format!("[SEND] {}\n", custom_debug(message)).green());
}

pub fn recv(message: &MumbleMessage) {
    println!("{}", format!("[RECV] {}\n", custom_debug(message)).red());
}

pub fn custom_debug(message: &MumbleMessage) -> String {
    #[allow(unused_variables)]
        match message {
        MumbleMessage::Version(m) => {
            let mut output = String::from("");
            output += "Version\n";

            if let Some(version) = &m.version {
                let major = (version & 0xffff0000) >> 16;
                let minor = (version & 0x0000ff00) >> 8;
                let patch = version & 0x000000ff;
                output += &format!("version: {}.{}.{} ", major, minor, patch);
            }

            if let Some(release) = &m.release {
                output += &format!("({}) ", release);
            }

            if m.os.is_some() && m.os_version.is_some() {
                let os = m.os.as_ref().unwrap();
                let os_version = m.os_version.as_ref().unwrap();
                output += &format!("({} / {})", os, os_version);
            }
            else if let Some(os) = &m.os {
                output += &format!("| OS: {} ", os);
            }
            else if let Some(os_version) = &m.os_version {
                output += &format!("| OS Version: {} ", os_version);
            }

            output
        },
        MumbleMessage::UdpTunnel(m) => format!("UdpTunnel"),
        MumbleMessage::Authenticate(m) => {
            let mut output = String::from("");
            output += "Authenticate\n";
            if let Some(username) = &m.username {
                output += &format!("Username: {}", username);
            }
            output
        },
        MumbleMessage::Ping(m) => format!("Ping"),
        MumbleMessage::Reject(m) => format!("Reject"),
        MumbleMessage::ServerSync(m) => {
            format!("ServerSync\n{:?}", m)
        },
        MumbleMessage::ChannelRemove(m) => format!("ChannelRemove"),
        MumbleMessage::ChannelState(m) => format!("ChannelState"),
        MumbleMessage::UserRemove(m) => format!("UserRemove"),
        MumbleMessage::UserState(m) => format!("UserState"),
        MumbleMessage::BanList(m) => format!("BanList"),
        MumbleMessage::TextMessage(m) => format!("TextMessage"),
        MumbleMessage::PermissionDenied(m) => format!("PermissionDenied"),
        MumbleMessage::Acl(m) => format!("Acl"),
        MumbleMessage::QueryUsers(m) => format!("QueryUsers"),
        MumbleMessage::CryptSetup(m) => format!("CryptSetup"),
        MumbleMessage::ContextActionModify(m) => format!("ContextActionModify"),
        MumbleMessage::ContextAction(m) => format!("ContextAction"),
        MumbleMessage::UserList(m) => format!("UserList"),
        MumbleMessage::VoiceTarget(m) => format!("VoiceTarget"),
        MumbleMessage::PermissionQuery(m) => format!("PermissionQuery"),
        MumbleMessage::CodecVersion(m) => format!("CodecVersion"),
        MumbleMessage::UserStats(m) => format!("UserStats"),
        MumbleMessage::RequestBlob(m) => format!("RequestBlob"),
        MumbleMessage::ServerConfig(m) => format!("ServerConfig"),
        MumbleMessage::SuggestConfig(m) => format!("SuggestConfig"),
    }
}