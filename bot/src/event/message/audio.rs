use line_bot_sdk::{
    error::AppError,
    models::{message::text::TextMessage, message::MessageObject, webhook_event::Audio},
};

pub fn handler(message: &Audio) -> Result<Option<Vec<MessageObject>>, AppError> {
    /* Ok(Some(vec![MessageObject::Text(TextMessage::new(
        "音声を受け取りました！".to_string(),
    ))])) */
    Ok(Some(vec![TextMessage::builder()
        .text("音声を受け取りました！")
        .build()
        .into()]))
}
