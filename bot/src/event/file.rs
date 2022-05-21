use line_bot_sdk::{
    error::AppError,
    models::{message::text::TextMessage, message::MessageObject, webhook_event::File},
};

pub fn handler(message: &File) -> Result<Vec<MessageObject>, AppError> {
    println!("{:?}", message);
    Ok(vec![MessageObject::Text(TextMessage::new(
        "ファイルを受け取りました！".to_string(),
    ))])
}
