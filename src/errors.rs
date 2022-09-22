pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Convert(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Fmt(#[from] std::fmt::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Elephantry(#[from] elephantry::Error),
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
}
