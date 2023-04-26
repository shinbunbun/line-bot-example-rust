use line_bot_sdk::models::{
    message::text::TextMessage, message::MessageObject, webhook_event::Image,
};

use crate::error::AppError;

pub fn handler(message: &Image) -> Result<Option<Vec<MessageObject>>, AppError> {
    Ok(Some(vec![TextMessage::builder()
        .text(&format!(
            "画像を受け取りました！\nメッセージID: {}",
            message.id
        ))
        .build()
        .into()]))
}
