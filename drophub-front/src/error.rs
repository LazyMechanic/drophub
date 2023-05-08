use std::rc::Rc;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseUrl(#[from] url::ParseError),
    #[error(transparent)]
    Jsonrpsee(#[from] Rc<jsonrpsee::core::Error>),
    #[error("Channel closed")]
    ChannelClosed { details: String },
    #[error("Received unexpected response")]
    ReceivedUnexpectedResponse { act: String, exp: String },
}

impl From<jsonrpsee::core::Error> for Error {
    fn from(value: jsonrpsee::core::Error) -> Self {
        Rc::new(value).into()
    }
}
