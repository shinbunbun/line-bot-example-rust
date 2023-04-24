#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    LineBotSdkError(line_bot_sdk::Error),
    IOError(std::io::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::BadRequest(errors) => write!(f, "Bad Request: {}", errors),
            AppError::LineBotSdkError(errors) => write!(f, "line bot sdk error: {}", errors),
            AppError::IOError(errors) => write!(f, "io error: {}", errors),
        }
    }
}
