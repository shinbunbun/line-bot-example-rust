use line_bot_sdk::models::{
    message::text::TextMessage, message::MessageObject, webhook_event::Video,
};

use crate::error::AppError;

pub fn handler(message: &Video) -> Result<Option<Vec<MessageObject>>, AppError> {
    Ok(Some(vec![TextMessage::builder()
        .text(&format!(
            "動画を受け取りました！\nメッセージID: {}",
            message.id
        ))
        .build()
        .into()]))
}
