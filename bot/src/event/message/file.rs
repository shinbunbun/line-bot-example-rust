use line_bot_sdk::{
    error::AppError,
    models::{message::text::TextMessage, message::MessageObject, webhook_event::File},
};

pub fn handler(message: &File) -> Result<Option<Vec<MessageObject>>, AppError> {
    /* Ok(Some(vec![MessageObject::Text(TextMessage::new(
        "ファイルを受け取りました！".to_string(),
    ))])) */
    Ok(Some(vec![TextMessage::builder()
        .text("ファイルを受け取りました！")
        .build()
        .into()]))
}
