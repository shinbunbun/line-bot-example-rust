use line_bot_sdk::{
    error::AppError,
    models::message::{text::TextMessage, MessageObject},
};

pub async fn index() -> Result<Option<Vec<MessageObject>>, AppError> {
    Ok(Some(vec![MessageObject::Text(TextMessage::new(
        "招待ありがと!!".to_string(),
    ))]))
}
