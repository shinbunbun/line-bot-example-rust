use line_bot_sdk::{
    error::AppError,
    models::{message::text::TextMessage, message::MessageObject, webhook_event::Location},
};

pub fn handler(message: &Location) -> Result<Option<Vec<MessageObject>>, AppError> {
    println!("{:?}", message);
    Ok(Some(vec![MessageObject::Text(TextMessage::new(format!(
        "受け取った住所: {}",
        message.address
    )))]))
}
