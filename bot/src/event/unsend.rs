use line_bot_sdk::{
    error::AppError,
    models::{
        message::{text::TextMessage, MessageObject},
        webhook_event::Event,
    },
};

pub async fn index(event: &Event) -> Result<Vec<MessageObject>, AppError> {
    let unsend = event
        .unsend
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Message not found".to_string()))?;
    Ok(vec![MessageObject::Text(TextMessage::new(format!(
        "メッセージが取り消されました！\n取り消されたmessageId: {}",
        unsend.message_id
    )))])
}
