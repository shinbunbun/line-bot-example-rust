use line_bot_sdk::{
    error::AppError,
    models::{message::text::TextMessage, message::MessageObject, webhook_event::Video},
};

pub fn handler(message: &Video) -> Result<Option<Vec<MessageObject>>, AppError> {
    println!("{:?}", message);
    Ok(Some(vec![MessageObject::Text(TextMessage::new(
        "動画を受け取りました！".to_string(),
    ))]))
}
