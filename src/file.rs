use actix_web::web::Bytes;
use async_trait::async_trait;
use line_bot_sdk::Client;

use crate::error::AppError;

#[async_trait(?Send)]
pub trait SaveFile {
    fn save_file(&self, bytes: &Bytes, path: &str) -> Result<(), AppError>;
    async fn get_content(&self, client: Client, message_id: &str) -> Result<Bytes, AppError>;
}

pub struct SaveLocalFile;

impl SaveLocalFile {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait(?Send)]
impl SaveFile for SaveLocalFile {
    fn save_file(&self, bytes: &Bytes, path: &str) -> Result<(), AppError> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path).map_err(AppError::IOError)?;
        file.write_all(bytes).map_err(AppError::IOError)?;
        Ok(())
    }

    async fn get_content(&self, client: Client, message_id: &str) -> Result<Bytes, AppError> {
        client
            .get_content(message_id)
            .await
            .map_err(AppError::LineBotSdkError)
    }
}
