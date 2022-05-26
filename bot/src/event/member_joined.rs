use line_bot_sdk::{
    error::AppError,
    models::{
        message::{text::TextMessage, MessageObject},
        webhook_event::Event,
    },
};

pub async fn index(event: &Event) -> Result<Option<Vec<MessageObject>>, AppError> {
    Ok(Some(vec![TextMessage::builder()
        .text(&format!(
            "ユーザーが参加しました！\n参加したユーザー: {}",
            event
                .joined
                .as_ref()
                .ok_or_else(|| AppError::BadRequest("joined not found".to_string()))?
                .members[0]
                .user_id
                .as_ref()
                .ok_or_else(|| AppError::BadRequest("userId not found".to_string()))?
        ))
        .build()
        .into()]))
}
