pub type JigglerResult<T> = Result<T, JigglerError>;

#[derive(Debug, thiserror::Error)]
pub enum JigglerError {
    #[error("error parsing duration string: {0}")]
    DurationParseError(String),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    EnigoInputError(#[from] enigo::InputError),

    #[error(transparent)]
    EnigoConError(#[from] enigo::NewConError),
}
