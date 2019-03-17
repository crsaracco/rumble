#[derive(Debug)]
pub enum RumbleError {
    IoError(std::io::Error),
    ProstEncodeError(prost::EncodeError),
    ProstDecodeError(prost::DecodeError),
    InvalidMessageId,
    NotEnoughBytesToDecode,
}

impl std::convert::From<prost::EncodeError> for RumbleError {
    fn from(prost_error: prost::EncodeError) -> Self {
        RumbleError::ProstEncodeError(prost_error)
    }
}

impl std::convert::From<prost::DecodeError> for RumbleError {
    fn from(prost_error: prost::DecodeError) -> Self {
        RumbleError::ProstDecodeError(prost_error)
    }
}

impl std::convert::From<std::io::Error> for RumbleError {
    fn from(io_error: std::io::Error) -> Self {
        RumbleError::IoError(io_error)
    }
}