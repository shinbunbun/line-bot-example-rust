use line_bot_sdk::{
    error::AppError,
    models::{message::text::TextMessage, message::MessageObject, webhook_event::File},
};

pub fn handler(message: &File) -> Result<Option<Vec<MessageObject>>, AppError> {
    Ok(Some(vec![TextMessage::builder()
        .text(&format!(
            "ファイルを受け取りました！\nメッセージID: {}",
            message.id
        ))
        .build()
        .into()]))
}
