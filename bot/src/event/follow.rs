use line_bot_sdk::models::message::{text::TextMessage, MessageObject};

use crate::error::AppError;

pub async fn index() -> Result<Option<Vec<MessageObject>>, AppError> {
    Ok(Some(vec![TextMessage::builder()
        .text("友達追加ありがとうございます！")
        .build()
        .into()]))
}
