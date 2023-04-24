use actix_web::web::Bytes;
use async_trait::async_trait;

use crate::error::AppError;

#[async_trait(?Send)]
pub trait SaveFile {
    fn save_file(&self, bytes: &Bytes, file_name: &str) -> Result<(), AppError>;
}

pub struct SaveLocalFile;

impl SaveLocalFile {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait(?Send)]
impl SaveFile for SaveLocalFile {
    fn save_file(&self, bytes: &Bytes, file_name: &str) -> Result<(), AppError> {
        use std::fs::File;
        use std::io::Write;

        let mut file =
            File::create(format!("./download/{}", file_name)).map_err(AppError::IOError)?;
        file.write_all(bytes).map_err(AppError::IOError)?;
        Ok(())
    }
}
