#[derive(Debug)]
pub enum FrameworkError {
    NotFound(String),
    Unauthorized(String),
    InternalError(String),
}

impl std::fmt::Display for FrameworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameworkError::NotFound(ref msg) => write!(f, "Not Found: {}", msg),
            FrameworkError::Unauthorized(ref msg) => write!(f, "Unauthorized: {}", msg),
            FrameworkError::InternalError(ref msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl std::error::Error for FrameworkError {}
