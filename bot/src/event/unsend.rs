use line_bot_sdk::models::{message::MessageObject, webhook_event::Event};

use crate::error::AppError;

pub async fn index(event: &Event) -> Result<Option<Vec<MessageObject>>, AppError> {
    let unsend = event
        .unsend
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("unsend not found".to_string()))?;
    println!(
        "メッセージが取り消されました！\n取り消されたmessageId: {}",
        unsend.message_id
    );
    Ok(None)
}
