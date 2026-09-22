use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum AppError {
    InvalidFormat,
    InvalidInput,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "Invalid format"),
            Self::InvalidInput => write!(f, "Invalid input"),
        }
    }
}
