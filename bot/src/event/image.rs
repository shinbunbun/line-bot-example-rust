use line_bot_sdk::{
    error::AppError,
    models::{message::text::TextMessage, message::MessageObject, webhook_event::Image},
};

pub fn handler(message: &Image) -> Result<Vec<MessageObject>, AppError> {
    println!("{:?}", message);
    Ok(vec![MessageObject::Text(TextMessage::new(
        "画像を受け取りました！".to_string(),
    ))])
}
