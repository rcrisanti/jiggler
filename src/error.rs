#[derive(Debug, thiserror::Error)]
pub enum JigglerError {
    #[error("error parsing duration string: {0}")]
    DurationParseError(String),
    
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    
    #[error(transparent)]
    MouseError(#[from] mouce::error::Error),
}
