#[derive(Debug)]
pub enum SasayakuError {
    IoError(std::io::Error),
    ProstEncodeError(prost::EncodeError),
    ProstDecodeError(prost::DecodeError),
    InvalidMessageId,
    NotEnoughBytesToDecode,
}

impl std::convert::From<prost::EncodeError> for SasayakuError {
    fn from(prost_error: prost::EncodeError) -> Self {
        SasayakuError::ProstEncodeError(prost_error)
    }
}

impl std::convert::From<prost::DecodeError> for SasayakuError {
    fn from(prost_error: prost::DecodeError) -> Self {
        SasayakuError::ProstDecodeError(prost_error)
    }
}

impl std::convert::From<std::io::Error> for SasayakuError {
    fn from(io_error: std::io::Error) -> Self {
        SasayakuError::IoError(io_error)
    }
}