use line_bot_sdk::{
    error::AppError,
    models::{message::MessageObject, webhook_event::Event},
};

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
