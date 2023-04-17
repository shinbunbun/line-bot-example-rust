use line_bot_sdk::models::{
    message::text::TextMessage, message::MessageObject, webhook_event::Audio,
};

use crate::{app_context::AppContext, error::AppError};

pub async fn handler(
    app_context: &AppContext,
    message: &Audio,
) -> Result<Option<Vec<MessageObject>>, AppError> {
    let bytes = app_context
        .line_client
        .get_message_content(&message.id)
        .await
        .map_err(AppError::LineBotSdkError)?;
    app_context
        .save_file
        .save_file(&bytes, &format!("{}.mp3", message.id))?;
    Ok(Some(vec![TextMessage::builder()
        .text(&format!(
            "音声を受け取りました！\nメッセージID: {}",
            message.id
        ))
        .build()
        .into()]))
}
