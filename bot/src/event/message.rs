use line_bot_sdk::{
    error::AppError,
    models::{
        message::MessageObject,
        webhook_event::{Event, Message},
    },
};

use super::text::text_event;

pub fn index(event: &Event) -> Result<Vec<MessageObject>, AppError> {
    let message = event
        .message
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Message not found".to_string()))?;
    match message {
        Message::Text(x) => text_event(x),
    }
}
