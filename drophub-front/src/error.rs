use std::rc::Rc;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseUrl(#[from] url::ParseError),
    #[error(transparent)]
    Jsonrpsee(#[from] jsonrpsee::core::Error),
    #[error(transparent)]
    HumantimeParse(#[from] humantime::DurationError),
    #[error(transparent)]
    TimeConversionRange(#[from] time::error::ConversionRange),
    #[error("Channel closed")]
    ChannelClosed { details: String },
    #[error("Received unexpected response")]
    ReceivedUnexpectedResponse { act: String, exp: String },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ShareError = Rc<Error>;
