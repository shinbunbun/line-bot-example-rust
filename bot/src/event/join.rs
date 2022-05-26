use line_bot_sdk::models::message::{text::TextMessage, MessageObject};

use crate::error::AppError;

pub async fn index() -> Result<Option<Vec<MessageObject>>, AppError> {
    Ok(Some(vec![TextMessage::builder()
        .text("招待ありがと!!")
        .build()
        .into()]))
}
