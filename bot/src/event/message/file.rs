use line_bot_sdk::models::{
    message::text::TextMessage, message::MessageObject, webhook_event::File,
};

use crate::error::AppError;

pub fn handler(message: &File) -> Result<Option<Vec<MessageObject>>, AppError> {
    Ok(Some(vec![TextMessage::builder()
        .text(&format!(
            "ファイルを受け取りました！\nメッセージID: {}",
            message.id
        ))
        .build()
        .into()]))
}
