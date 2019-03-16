//! THIS FILE IS GENERATED FROM `mumble.proto`.
//! THIS MEANS THAT IT IS SUBJECT TO THE 3-CLAUSE BSD LICENSE OF THE MUMBLE PROJECT!
//! SEE `/LICENSE-mumble` FOR MORE INFORMATION!
//!
//! This file is only included in the repository for now for ease-of-development.
//! It will go away eventually.

#[derive(Clone, PartialEq, Message)]
pub struct Version {
    /// 2-byte Major, 1-byte Minor and 1-byte Patch version number.
    #[prost(uint32, optional, tag = "1")]
    pub version: ::std::option::Option<u32>,
    /// Client release name.
    #[prost(string, optional, tag = "2")]
    pub release: ::std::option::Option<String>,
    /// Client OS name.
    #[prost(string, optional, tag = "3")]
    pub os: ::std::option::Option<String>,
    /// Client OS version.
    #[prost(string, optional, tag = "4")]
    pub os_version: ::std::option::Option<String>,
}
/// Not used. Not even for tunneling UDP through TCP.
#[derive(Clone, PartialEq, Message)]
pub struct UdpTunnel {
    /// Not used.
    #[prost(bytes, required, tag = "1")]
    pub packet: Vec<u8>,
}
/// Used by the client to send the authentication credentials to the server.
#[derive(Clone, PartialEq, Message)]
pub struct Authenticate {
    /// UTF-8 encoded username.
    #[prost(string, optional, tag = "1")]
    pub username: ::std::option::Option<String>,
    /// Server or user password.
    #[prost(string, optional, tag = "2")]
    pub password: ::std::option::Option<String>,
    /// Additional access tokens for server ACL groups.
    #[prost(string, repeated, tag = "3")]
    pub tokens: ::std::vec::Vec<String>,
    /// A list of CELT bitstream version constants supported by the client.
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub celt_versions: ::std::vec::Vec<i32>,
    #[prost(bool, optional, tag = "5", default = "false")]
    pub opus: ::std::option::Option<bool>,
}
/// Sent by the client to notify the server that the client is still alive.
/// Server must reply to the packet with the same timestamp and its own
/// good/late/lost/resync numbers. None of the fields is strictly required.
#[derive(Clone, PartialEq, Message)]
pub struct Ping {
    /// Client timestamp. Server should not attempt to decode.
    #[prost(uint64, optional, tag = "1")]
    pub timestamp: ::std::option::Option<u64>,
    /// The amount of good packets received.
    #[prost(uint32, optional, tag = "2")]
    pub good: ::std::option::Option<u32>,
    /// The amount of late packets received.
    #[prost(uint32, optional, tag = "3")]
    pub late: ::std::option::Option<u32>,
    /// The amount of packets never received.
    #[prost(uint32, optional, tag = "4")]
    pub lost: ::std::option::Option<u32>,
    /// The amount of nonce resyncs.
    #[prost(uint32, optional, tag = "5")]
    pub resync: ::std::option::Option<u32>,
    /// The total amount of UDP packets received.
    #[prost(uint32, optional, tag = "6")]
    pub udp_packets: ::std::option::Option<u32>,
    /// The total amount of TCP packets received.
    #[prost(uint32, optional, tag = "7")]
    pub tcp_packets: ::std::option::Option<u32>,
    /// UDP ping average.
    #[prost(float, optional, tag = "8")]
    pub udp_ping_avg: ::std::option::Option<f32>,
    /// UDP ping variance.
    #[prost(float, optional, tag = "9")]
    pub udp_ping_var: ::std::option::Option<f32>,
    /// TCP ping average.
    #[prost(float, optional, tag = "10")]
    pub tcp_ping_avg: ::std::option::Option<f32>,
    /// TCP ping variance.
    #[prost(float, optional, tag = "11")]
    pub tcp_ping_var: ::std::option::Option<f32>,
}
/// Sent by the server when it rejects the user connection.
#[derive(Clone, PartialEq, Message)]
pub struct Reject {
    /// Rejection type.
    #[prost(enumeration = "reject::RejectType", optional, tag = "1")]
    pub type_: ::std::option::Option<i32>,
    /// Human readable rejection reason.
    #[prost(string, optional, tag = "2")]
    pub reason: ::std::option::Option<String>,
}
pub mod reject {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    pub enum RejectType {
        /// The rejection reason is unknown (details should be available
        /// in Reject.reason).
        None = 0,
        /// The client attempted to connect with an incompatible version.
        WrongVersion = 1,
        /// The user name supplied by the client was invalid.
        InvalidUsername = 2,
        /// The client attempted to authenticate as a user with a password but it
        /// was wrong.
        WrongUserPw = 3,
        /// The client attempted to connect to a passworded server but the password
        /// was wrong.
        WrongServerPw = 4,
        /// Supplied username is already in use.
        UsernameInUse = 5,
        /// Server is currently full and cannot accept more users.
        ServerFull = 6,
        /// The user did not provide a certificate but one is required.
        NoCertificate = 7,
        AuthenticatorFail = 8,
    }
}
/// ServerSync message is sent by the server when it has authenticated the user
/// and finished synchronizing the server state.
#[derive(Clone, PartialEq, Message)]
pub struct ServerSync {
    /// The session of the current user.
    #[prost(uint32, optional, tag = "1")]
    pub session: ::std::option::Option<u32>,
    /// Maximum bandwidth that the user should use.
    #[prost(uint32, optional, tag = "2")]
    pub max_bandwidth: ::std::option::Option<u32>,
    /// Server welcome text.
    #[prost(string, optional, tag = "3")]
    pub welcome_text: ::std::option::Option<String>,
    /// Current user permissions in the root channel.
    #[prost(uint64, optional, tag = "4")]
    pub permissions: ::std::option::Option<u64>,
}
/// Sent by the client when it wants a channel removed. Sent by the server when
/// a channel has been removed and clients should be notified.
#[derive(Clone, PartialEq, Message)]
pub struct ChannelRemove {
    #[prost(uint32, required, tag = "1")]
    pub channel_id: u32,
}
/// Used to communicate channel properties between the client and the server.
/// Sent by the server during the login process or when channel properties are
/// updated. Client may use this message to update said channel properties.
#[derive(Clone, PartialEq, Message)]
pub struct ChannelState {
    /// Unique ID for the channel within the server.
    #[prost(uint32, optional, tag = "1")]
    pub channel_id: ::std::option::Option<u32>,
    /// channel_id of the parent channel.
    #[prost(uint32, optional, tag = "2")]
    pub parent: ::std::option::Option<u32>,
    /// UTF-8 encoded channel name.
    #[prost(string, optional, tag = "3")]
    pub name: ::std::option::Option<String>,
    /// A collection of channel id values of the linked channels. Absent during
    /// the first channel listing.
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub links: ::std::vec::Vec<u32>,
    /// UTF-8 encoded channel description. Only if the description is less than
    /// 128 bytes
    #[prost(string, optional, tag = "5")]
    pub description: ::std::option::Option<String>,
    /// A collection of channel_id values that should be added to links.
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub links_add: ::std::vec::Vec<u32>,
    /// A collection of channel_id values that should be removed from links.
    #[prost(uint32, repeated, packed = "false", tag = "7")]
    pub links_remove: ::std::vec::Vec<u32>,
    /// True if the channel is temporary.
    #[prost(bool, optional, tag = "8", default = "false")]
    pub temporary: ::std::option::Option<bool>,
    /// Position weight to tweak the channel position in the channel list.
    #[prost(int32, optional, tag = "9", default = "0")]
    pub position: ::std::option::Option<i32>,
    /// SHA1 hash of the description if the description is 128 bytes or more.
    #[prost(bytes, optional, tag = "10")]
    pub description_hash: ::std::option::Option<Vec<u8>>,
    /// Maximum number of users allowed in the channel. If this value is zero,
    /// the maximum number of users allowed in the channel is given by the
    /// server's "usersperchannel" setting.
    #[prost(uint32, optional, tag = "11")]
    pub max_users: ::std::option::Option<u32>,
}
/// Used to communicate user leaving or being kicked. May be sent by the client
/// when it attempts to kick a user. Sent by the server when it informs the
/// clients that a user is not present anymore.
#[derive(Clone, PartialEq, Message)]
pub struct UserRemove {
    /// The user who is being kicked, identified by their session, not present
    /// when no one is being kicked.
    #[prost(uint32, required, tag = "1")]
    pub session: u32,
    /// The user who initiated the removal. Either the user who performs the kick
    /// or the user who is currently leaving.
    #[prost(uint32, optional, tag = "2")]
    pub actor: ::std::option::Option<u32>,
    /// Reason for the kick, stored as the ban reason if the user is banned.
    #[prost(string, optional, tag = "3")]
    pub reason: ::std::option::Option<String>,
    /// True if the kick should result in a ban.
    #[prost(bool, optional, tag = "4")]
    pub ban: ::std::option::Option<bool>,
}
/// Sent by the server when it communicates new and changed users to client.
/// First seen during login procedure. May be sent by the client when it wishes
/// to alter its state.
#[derive(Clone, PartialEq, Message)]
pub struct UserState {
    /// Unique user session ID of the user whose state this is, may change on
    /// reconnect.
    #[prost(uint32, optional, tag = "1")]
    pub session: ::std::option::Option<u32>,
    /// The session of the user who is updating this user.
    #[prost(uint32, optional, tag = "2")]
    pub actor: ::std::option::Option<u32>,
    /// User name, UTF-8 encoded.
    #[prost(string, optional, tag = "3")]
    pub name: ::std::option::Option<String>,
    /// Registered user ID if the user is registered.
    #[prost(uint32, optional, tag = "4")]
    pub user_id: ::std::option::Option<u32>,
    /// Channel on which the user is.
    #[prost(uint32, optional, tag = "5")]
    pub channel_id: ::std::option::Option<u32>,
    /// True if the user is muted by admin.
    #[prost(bool, optional, tag = "6")]
    pub mute: ::std::option::Option<bool>,
    /// True if the user is deafened by admin.
    #[prost(bool, optional, tag = "7")]
    pub deaf: ::std::option::Option<bool>,
    /// True if the user has been suppressed from talking by a reason other than
    /// being muted.
    #[prost(bool, optional, tag = "8")]
    pub suppress: ::std::option::Option<bool>,
    /// True if the user has muted self.
    #[prost(bool, optional, tag = "9")]
    pub self_mute: ::std::option::Option<bool>,
    /// True if the user has deafened self.
    #[prost(bool, optional, tag = "10")]
    pub self_deaf: ::std::option::Option<bool>,
    /// User image if it is less than 128 bytes.
    #[prost(bytes, optional, tag = "11")]
    pub texture: ::std::option::Option<Vec<u8>>,
    /// The positional audio plugin identifier.
    /// Positional audio information is only sent to users who share
    /// identical plugin contexts.
    ///
    /// This value is not trasmitted to clients.
    #[prost(bytes, optional, tag = "12")]
    pub plugin_context: ::std::option::Option<Vec<u8>>,
    /// The user's plugin-specific identity.
    /// This value is not transmitted to clients.
    #[prost(string, optional, tag = "13")]
    pub plugin_identity: ::std::option::Option<String>,
    /// User comment if it is less than 128 bytes.
    #[prost(string, optional, tag = "14")]
    pub comment: ::std::option::Option<String>,
    /// The hash of the user certificate.
    #[prost(string, optional, tag = "15")]
    pub hash: ::std::option::Option<String>,
    /// SHA1 hash of the user comment if it 128 bytes or more.
    #[prost(bytes, optional, tag = "16")]
    pub comment_hash: ::std::option::Option<Vec<u8>>,
    /// SHA1 hash of the user picture if it 128 bytes or more.
    #[prost(bytes, optional, tag = "17")]
    pub texture_hash: ::std::option::Option<Vec<u8>>,
    /// True if the user is a priority speaker.
    #[prost(bool, optional, tag = "18")]
    pub priority_speaker: ::std::option::Option<bool>,
    /// True if the user is currently recording.
    #[prost(bool, optional, tag = "19")]
    pub recording: ::std::option::Option<bool>,
}
/// Relays information on the bans. The client may send the BanList message to
/// either modify the list of bans or query them from the server. The server
/// sends this list only after a client queries for it.
#[derive(Clone, PartialEq, Message)]
pub struct BanList {
    /// List of ban entries currently in place.
    #[prost(message, repeated, tag = "1")]
    pub bans: ::std::vec::Vec<ban_list::BanEntry>,
    /// True if the server should return the list, false if it should replace old
    /// ban list with the one provided.
    #[prost(bool, optional, tag = "2", default = "false")]
    pub query: ::std::option::Option<bool>,
}
pub mod ban_list {
    #[derive(Clone, PartialEq, Message)]
    pub struct BanEntry {
        /// Banned IP address.
        #[prost(bytes, required, tag = "1")]
        pub address: Vec<u8>,
        /// The length of the subnet mask for the ban.
        #[prost(uint32, required, tag = "2")]
        pub mask: u32,
        /// User name for identification purposes (does not affect the ban).
        #[prost(string, optional, tag = "3")]
        pub name: ::std::option::Option<String>,
        /// The certificate hash of the banned user.
        #[prost(string, optional, tag = "4")]
        pub hash: ::std::option::Option<String>,
        /// Reason for the ban (does not affect the ban).
        #[prost(string, optional, tag = "5")]
        pub reason: ::std::option::Option<String>,
        /// Ban start time.
        #[prost(string, optional, tag = "6")]
        pub start: ::std::option::Option<String>,
        /// Ban duration in seconds.
        #[prost(uint32, optional, tag = "7")]
        pub duration: ::std::option::Option<u32>,
    }
}
/// Used to send and broadcast text messages.
#[derive(Clone, PartialEq, Message)]
pub struct TextMessage {
    /// The message sender, identified by its session.
    #[prost(uint32, optional, tag = "1")]
    pub actor: ::std::option::Option<u32>,
    /// Target users for the message, identified by their session.
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub session: ::std::vec::Vec<u32>,
    /// The channels to which the message is sent, identified by their
    /// channel_ids.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub channel_id: ::std::vec::Vec<u32>,
    /// The root channels when sending message recursively to several channels,
    /// identified by their channel_ids.
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub tree_id: ::std::vec::Vec<u32>,
    /// The UTF-8 encoded message. May be HTML if the server allows.
    #[prost(string, required, tag = "5")]
    pub message: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct PermissionDenied {
    /// The denied permission when type is Permission.
    #[prost(uint32, optional, tag = "1")]
    pub permission: ::std::option::Option<u32>,
    /// channel_id for the channel where the permission was denied when type is
    /// Permission.
    #[prost(uint32, optional, tag = "2")]
    pub channel_id: ::std::option::Option<u32>,
    /// The user who was denied permissions, identified by session.
    #[prost(uint32, optional, tag = "3")]
    pub session: ::std::option::Option<u32>,
    /// Textual reason for the denial.
    #[prost(string, optional, tag = "4")]
    pub reason: ::std::option::Option<String>,
    /// Type of the denial.
    #[prost(enumeration = "permission_denied::DenyType", optional, tag = "5")]
    pub type_: ::std::option::Option<i32>,
    /// The name that is invalid when type is UserName.
    #[prost(string, optional, tag = "6")]
    pub name: ::std::option::Option<String>,
}
pub mod permission_denied {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    pub enum DenyType {
        /// Operation denied for other reason, see reason field.
        Text = 0,
        /// Permissions were denied.
        Permission = 1,
        /// Cannot modify SuperUser.
        SuperUser = 2,
        /// Invalid channel name.
        ChannelName = 3,
        /// Text message too long.
        TextTooLong = 4,
        /// The flux capacitor was spelled wrong.
        H9k = 5,
        /// Operation not permitted in temporary channel.
        TemporaryChannel = 6,
        /// Operation requires certificate.
        MissingCertificate = 7,
        /// Invalid username.
        UserName = 8,
        /// Channel is full.
        ChannelFull = 9,
        /// Channels are nested too deply.
        NestingLimit = 10,
        /// Maximum channel count reached.
        ChannelCountLimit = 11,
    }
}
#[derive(Clone, PartialEq, Message)]
pub struct Acl {
    /// Channel ID of the channel this message affects.
    #[prost(uint32, required, tag = "1")]
    pub channel_id: u32,
    /// True if the channel inherits its parent's ACLs.
    #[prost(bool, optional, tag = "2", default = "true")]
    pub inherit_acls: ::std::option::Option<bool>,
    /// User group specifications.
    #[prost(message, repeated, tag = "3")]
    pub groups: ::std::vec::Vec<acl::ChanGroup>,
    /// ACL specifications.
    #[prost(message, repeated, tag = "4")]
    pub acls: ::std::vec::Vec<acl::ChanAcl>,
    /// True if the message is a query for ACLs instead of setting them.
    #[prost(bool, optional, tag = "5", default = "false")]
    pub query: ::std::option::Option<bool>,
}
pub mod acl {
    #[derive(Clone, PartialEq, Message)]
    pub struct ChanGroup {
        /// Name of the channel group, UTF-8 encoded.
        #[prost(string, required, tag = "1")]
        pub name: String,
        /// True if the group has been inherited from the parent (Read only).
        #[prost(bool, optional, tag = "2", default = "true")]
        pub inherited: ::std::option::Option<bool>,
        /// True if the group members are inherited.
        #[prost(bool, optional, tag = "3", default = "true")]
        pub inherit: ::std::option::Option<bool>,
        /// True if the group can be inherited by sub channels.
        #[prost(bool, optional, tag = "4", default = "true")]
        pub inheritable: ::std::option::Option<bool>,
        /// Users explicitly included in this group, identified by user_id.
        #[prost(uint32, repeated, packed = "false", tag = "5")]
        pub add: ::std::vec::Vec<u32>,
        /// Users explicitly removed from this group in this channel if the group
        /// has been inherited, identified by user_id.
        #[prost(uint32, repeated, packed = "false", tag = "6")]
        pub remove: ::std::vec::Vec<u32>,
        /// Users inherited, identified by user_id.
        #[prost(uint32, repeated, packed = "false", tag = "7")]
        pub inherited_members: ::std::vec::Vec<u32>,
    }
    #[derive(Clone, PartialEq, Message)]
    pub struct ChanAcl {
        /// True if this ACL applies to the current channel.
        #[prost(bool, optional, tag = "1", default = "true")]
        pub apply_here: ::std::option::Option<bool>,
        /// True if this ACL applies to the sub channels.
        #[prost(bool, optional, tag = "2", default = "true")]
        pub apply_subs: ::std::option::Option<bool>,
        /// True if the ACL has been inherited from the parent.
        #[prost(bool, optional, tag = "3", default = "true")]
        pub inherited: ::std::option::Option<bool>,
        /// ID of the user that is affected by this ACL.
        #[prost(uint32, optional, tag = "4")]
        pub user_id: ::std::option::Option<u32>,
        /// ID of the group that is affected by this ACL.
        #[prost(string, optional, tag = "5")]
        pub group: ::std::option::Option<String>,
        /// Bit flag field of the permissions granted by this ACL.
        #[prost(uint32, optional, tag = "6")]
        pub grant: ::std::option::Option<u32>,
        /// Bit flag field of the permissions denied by this ACL.
        #[prost(uint32, optional, tag = "7")]
        pub deny: ::std::option::Option<u32>,
    }
}
/// Client may use this message to refresh its registered user information. The
/// client should fill the IDs or Names of the users it wants to refresh. The
/// server fills the missing parts and sends the message back.
#[derive(Clone, PartialEq, Message)]
pub struct QueryUsers {
    /// user_ids.
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub ids: ::std::vec::Vec<u32>,
    /// User names in the same order as ids.
    #[prost(string, repeated, tag = "2")]
    pub names: ::std::vec::Vec<String>,
}
/// Used to initialize and resync the UDP encryption. Either side may request a
/// resync by sending the message without any values filled. The resync is
/// performed by sending the message with only the client or server nonce
/// filled.
#[derive(Clone, PartialEq, Message)]
pub struct CryptSetup {
    /// Encryption key.
    #[prost(bytes, optional, tag = "1")]
    pub key: ::std::option::Option<Vec<u8>>,
    /// Client nonce.
    #[prost(bytes, optional, tag = "2")]
    pub client_nonce: ::std::option::Option<Vec<u8>>,
    /// Server nonce.
    #[prost(bytes, optional, tag = "3")]
    pub server_nonce: ::std::option::Option<Vec<u8>>,
}
#[derive(Clone, PartialEq, Message)]
pub struct ContextActionModify {
    /// The action name.
    #[prost(string, required, tag = "1")]
    pub action: String,
    /// The display name of the action.
    #[prost(string, optional, tag = "2")]
    pub text: ::std::option::Option<String>,
    /// Context bit flags defining where the action should be displayed.
    #[prost(uint32, optional, tag = "3")]
    pub context: ::std::option::Option<u32>,
    #[prost(enumeration = "context_action_modify::Operation", optional, tag = "4")]
    pub operation: ::std::option::Option<i32>,
}
pub mod context_action_modify {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    pub enum Context {
        /// Action is applicable to the server.
        Server = 1,
        /// Action can target a Channel.
        Channel = 2,
        /// Action can target a User.
        User = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    pub enum Operation {
        Add = 0,
        Remove = 1,
    }
}
/// Sent by the client when it wants to initiate a Context action.
#[derive(Clone, PartialEq, Message)]
pub struct ContextAction {
    /// The target User for the action, identified by session.
    #[prost(uint32, optional, tag = "1")]
    pub session: ::std::option::Option<u32>,
    /// The target Channel for the action, identified by channel_id.
    #[prost(uint32, optional, tag = "2")]
    pub channel_id: ::std::option::Option<u32>,
    /// The action that should be executed.
    #[prost(string, required, tag = "3")]
    pub action: String,
}
/// Lists the registered users.
#[derive(Clone, PartialEq, Message)]
pub struct UserList {
    /// A list of registered users.
    #[prost(message, repeated, tag = "1")]
    pub users: ::std::vec::Vec<user_list::User>,
}
pub mod user_list {
    #[derive(Clone, PartialEq, Message)]
    pub struct User {
        /// Registered user ID.
        #[prost(uint32, required, tag = "1")]
        pub user_id: u32,
        /// Registered user name.
        #[prost(string, optional, tag = "2")]
        pub name: ::std::option::Option<String>,
        #[prost(string, optional, tag = "3")]
        pub last_seen: ::std::option::Option<String>,
        #[prost(uint32, optional, tag = "4")]
        pub last_channel: ::std::option::Option<u32>,
    }
}
/// Sent by the client when it wants to register or clear whisper targets.
///
/// Note: The first available target ID is 1 as 0 is reserved for normal
/// talking. Maximum target ID is 30.
#[derive(Clone, PartialEq, Message)]
pub struct VoiceTarget {
    /// Voice target ID.
    #[prost(uint32, optional, tag = "1")]
    pub id: ::std::option::Option<u32>,
    /// The receivers that this voice target includes.
    #[prost(message, repeated, tag = "2")]
    pub targets: ::std::vec::Vec<voice_target::Target>,
}
pub mod voice_target {
    #[derive(Clone, PartialEq, Message)]
    pub struct Target {
        /// Users that are included as targets.
        #[prost(uint32, repeated, packed = "false", tag = "1")]
        pub session: ::std::vec::Vec<u32>,
        /// Channel that is included as a target.
        #[prost(uint32, optional, tag = "2")]
        pub channel_id: ::std::option::Option<u32>,
        /// ACL group that is included as a target.
        #[prost(string, optional, tag = "3")]
        pub group: ::std::option::Option<String>,
        /// True if the voice should follow links from the specified channel.
        #[prost(bool, optional, tag = "4", default = "false")]
        pub links: ::std::option::Option<bool>,
        /// True if the voice should also be sent to children of the specific
        /// channel.
        #[prost(bool, optional, tag = "5", default = "false")]
        pub children: ::std::option::Option<bool>,
    }
}
/// Sent by the client when it wants permissions for a certain channel. Sent by
/// the server when it replies to the query or wants the user to resync all
/// channel permissions.
#[derive(Clone, PartialEq, Message)]
pub struct PermissionQuery {
    /// channel_id of the channel for which the permissions are queried.
    #[prost(uint32, optional, tag = "1")]
    pub channel_id: ::std::option::Option<u32>,
    /// Channel permissions.
    #[prost(uint32, optional, tag = "2")]
    pub permissions: ::std::option::Option<u32>,
    /// True if the client should drop its current permission information for all
    /// channels.
    #[prost(bool, optional, tag = "3", default = "false")]
    pub flush: ::std::option::Option<bool>,
}
/// Sent by the server to notify the users of the version of the CELT codec they
/// should use. This may change during the connection when new users join.
#[derive(Clone, PartialEq, Message)]
pub struct CodecVersion {
    /// The version of the CELT Alpha codec.
    #[prost(int32, required, tag = "1")]
    pub alpha: i32,
    /// The version of the CELT Beta codec.
    #[prost(int32, required, tag = "2")]
    pub beta: i32,
    /// True if the user should prefer Alpha over Beta.
    #[prost(bool, required, tag = "3", default = "true")]
    pub prefer_alpha: bool,
    #[prost(bool, optional, tag = "4", default = "false")]
    pub opus: ::std::option::Option<bool>,
}
/// Used to communicate user stats between the server and clients.
#[derive(Clone, PartialEq, Message)]
pub struct UserStats {
    /// User whose stats these are.
    #[prost(uint32, optional, tag = "1")]
    pub session: ::std::option::Option<u32>,
    /// True if the message contains only mutable stats (packets, ping).
    #[prost(bool, optional, tag = "2", default = "false")]
    pub stats_only: ::std::option::Option<bool>,
    /// Full user certificate chain of the user certificate in DER format.
    #[prost(bytes, repeated, tag = "3")]
    pub certificates: ::std::vec::Vec<Vec<u8>>,
    /// Packet statistics for packets received from the client.
    #[prost(message, optional, tag = "4")]
    pub from_client: ::std::option::Option<user_stats::Stats>,
    /// Packet statistics for packets sent by the server.
    #[prost(message, optional, tag = "5")]
    pub from_server: ::std::option::Option<user_stats::Stats>,
    /// Amount of UDP packets sent.
    #[prost(uint32, optional, tag = "6")]
    pub udp_packets: ::std::option::Option<u32>,
    /// Amount of TCP packets sent.
    #[prost(uint32, optional, tag = "7")]
    pub tcp_packets: ::std::option::Option<u32>,
    /// UDP ping average.
    #[prost(float, optional, tag = "8")]
    pub udp_ping_avg: ::std::option::Option<f32>,
    /// UDP ping variance.
    #[prost(float, optional, tag = "9")]
    pub udp_ping_var: ::std::option::Option<f32>,
    /// TCP ping average.
    #[prost(float, optional, tag = "10")]
    pub tcp_ping_avg: ::std::option::Option<f32>,
    /// TCP ping variance.
    #[prost(float, optional, tag = "11")]
    pub tcp_ping_var: ::std::option::Option<f32>,
    /// Client version.
    #[prost(message, optional, tag = "12")]
    pub version: ::std::option::Option<Version>,
    /// A list of CELT bitstream version constants supported by the client of this
    /// user.
    #[prost(int32, repeated, packed = "false", tag = "13")]
    pub celt_versions: ::std::vec::Vec<i32>,
    /// Client IP address.
    #[prost(bytes, optional, tag = "14")]
    pub address: ::std::option::Option<Vec<u8>>,
    /// Bandwith used by this client.
    #[prost(uint32, optional, tag = "15")]
    pub bandwidth: ::std::option::Option<u32>,
    /// Connection duration.
    #[prost(uint32, optional, tag = "16")]
    pub onlinesecs: ::std::option::Option<u32>,
    /// Duration since last activity.
    #[prost(uint32, optional, tag = "17")]
    pub idlesecs: ::std::option::Option<u32>,
    /// True if the user has a strong certificate.
    #[prost(bool, optional, tag = "18", default = "false")]
    pub strong_certificate: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "19", default = "false")]
    pub opus: ::std::option::Option<bool>,
}
pub mod user_stats {
    #[derive(Clone, PartialEq, Message)]
    pub struct Stats {
        /// The amount of good packets received.
        #[prost(uint32, optional, tag = "1")]
        pub good: ::std::option::Option<u32>,
        /// The amount of late packets received.
        #[prost(uint32, optional, tag = "2")]
        pub late: ::std::option::Option<u32>,
        /// The amount of packets never received.
        #[prost(uint32, optional, tag = "3")]
        pub lost: ::std::option::Option<u32>,
        /// The amount of nonce resyncs.
        #[prost(uint32, optional, tag = "4")]
        pub resync: ::std::option::Option<u32>,
    }
}
/// Used by the client to request binary data from the server. By default large
/// comments or textures are not sent within standard messages but instead the
/// hash is. If the client does not recognize the hash it may request the
/// resource when it needs it. The client does so by sending a RequestBlob
/// message with the correct fields filled with the user sessions or channel_ids
/// it wants to receive. The server replies to this by sending a new
/// UserState/ChannelState message with the resources filled even if they would
/// normally be transmitted as hashes.
#[derive(Clone, PartialEq, Message)]
pub struct RequestBlob {
    /// sessions of the requested UserState textures.
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub session_texture: ::std::vec::Vec<u32>,
    /// sessions of the requested UserState comments.
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub session_comment: ::std::vec::Vec<u32>,
    /// channel_ids of the requested ChannelState descriptions.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub channel_description: ::std::vec::Vec<u32>,
}
/// Sent by the server when it informs the clients on server configuration
/// details.
#[derive(Clone, PartialEq, Message)]
pub struct ServerConfig {
    /// The maximum bandwidth the clients should use.
    #[prost(uint32, optional, tag = "1")]
    pub max_bandwidth: ::std::option::Option<u32>,
    /// Server welcome text.
    #[prost(string, optional, tag = "2")]
    pub welcome_text: ::std::option::Option<String>,
    /// True if the server allows HTML.
    #[prost(bool, optional, tag = "3")]
    pub allow_html: ::std::option::Option<bool>,
    /// Maximum text message length.
    #[prost(uint32, optional, tag = "4")]
    pub message_length: ::std::option::Option<u32>,
    /// Maximum image message length.
    #[prost(uint32, optional, tag = "5")]
    pub image_message_length: ::std::option::Option<u32>,
    /// The maximum number of users allowed on the server.
    #[prost(uint32, optional, tag = "6")]
    pub max_users: ::std::option::Option<u32>,
}
/// Sent by the server to inform the clients of suggested client configuration
/// specified by the server administrator.
#[derive(Clone, PartialEq, Message)]
pub struct SuggestConfig {
    /// Suggested client version.
    #[prost(uint32, optional, tag = "1")]
    pub version: ::std::option::Option<u32>,
    /// True if the administrator suggests positional audio to be used on this
    /// server.
    #[prost(bool, optional, tag = "2")]
    pub positional: ::std::option::Option<bool>,
    /// True if the administrator suggests push to talk to be used on this server.
    #[prost(bool, optional, tag = "3")]
    pub push_to_talk: ::std::option::Option<bool>,
}
