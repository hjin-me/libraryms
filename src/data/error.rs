use std::fmt;

struct ErrorInner {
    msg: String,
    cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

pub struct Error(Box<ErrorInner>);

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("msg", &self.0.msg)
            .field("cause", &self.0.cause)
            .finish()
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.msg)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.cause.as_ref().map(|e| &**e as _)
    }
}

pub fn with_msg<E>(err: Option<E>, msg: &str) -> Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    Error(Box::new(ErrorInner {
        msg: msg.to_string(),
        cause: err.map(Into::into),
    }))
}
