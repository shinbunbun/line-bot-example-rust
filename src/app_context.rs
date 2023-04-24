use std::sync::Arc;

use line_bot_sdk::Client;

use crate::file::SaveFile;

pub struct AppContext {
    pub line_client: Arc<Client>,
    pub destination: Arc<Box<dyn SaveFile>>,
}

impl AppContext {
    pub fn new(line_client: Arc<Client>, destination: Arc<Box<dyn SaveFile>>) -> Self {
        Self {
            line_client,
            destination,
        }
    }
}
