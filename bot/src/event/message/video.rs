use line_bot_sdk::{
    error::AppError,
    models::{message::text::TextMessage, message::MessageObject, webhook_event::Video},
};

pub fn handler(message: &Video) -> Result<Option<Vec<MessageObject>>, AppError> {
    Ok(Some(vec![TextMessage::builder()
        .text(&format!(
            "動画を受け取りました！\nメッセージID: {}",
            message.id
        ))
        .build()
        .into()]))
}
