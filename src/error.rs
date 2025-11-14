use reqwest::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // Foreign errors - using transparent for cleaner error forwarding
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::error::Error),

    #[error(transparent)]
    FormEncoding(#[from] serde_html_form::ser::Error),

    #[error(transparent)]
    UTF8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    // WS/request errors
    #[error("Connection was closed: {0}")]
    Closed(String),

    #[error("HTTP Status: '{0}'")]
    Status(StatusCode),

    #[error("HTTP Status: '{0}' Message: {1}")]
    StatusText(StatusCode, String),

    #[error("{0} Retry in: '{1:?}'")]
    Limited(StatusCode, Option<i64>),

    #[error("{0} {1}")]
    Tungstenite(Box<tokio_tungstenite::tungstenite::Error>, String),

    #[error("Webex API changed: {0}")]
    Api(&'static str),

    #[error("Authentication error")]
    Authentication,

    #[error("{0}")]
    UserError(String),

    // catch-all
    #[error("{0}")]
    Other(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}
