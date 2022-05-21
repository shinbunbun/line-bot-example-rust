use line_bot_sdk::{
    error::AppError,
    models::{message::text::TextMessage, message::MessageObject, webhook_event::Audio},
};

pub fn handler(message: &Audio) -> Result<Vec<MessageObject>, AppError> {
    println!("{:?}", message);
    Ok(vec![MessageObject::Text(TextMessage::new(
        "音声を受け取りました！".to_string(),
    ))])
}
