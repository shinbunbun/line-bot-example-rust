use line_bot_sdk::models::{message::MessageObject, webhook_event::Event};

use crate::error::AppError;

pub async fn index(event: &Event) -> Result<Option<Vec<MessageObject>>, AppError> {
    println!(
        "unfollowされました...\nuserId: {}",
        event
            .source
            .user_id
            .as_ref()
            .ok_or_else(|| AppError::BadRequest("userId not found".to_string()))?
    );
    Ok(None)
}
