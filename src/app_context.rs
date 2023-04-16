use std::sync::Arc;

use line_bot_sdk::Client;

use crate::file::SaveFile;

pub struct AppContext {
    pub line_client: Arc<Client>,
    pub save_file: Arc<Box<dyn SaveFile>>,
}

impl AppContext {
    pub fn new(line_client: Arc<Client>, save_file: Arc<Box<dyn SaveFile>>) -> Self {
        Self {
            line_client,
            save_file,
        }
    }
}
