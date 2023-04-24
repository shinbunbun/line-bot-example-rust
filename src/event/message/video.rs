use line_bot_sdk::models::{
    message::text::TextMessage, message::MessageObject, webhook_event::Video,
};

use crate::{app_context::AppContext, error::AppError};

pub async fn handler(
    app_context: &AppContext,
    message: &Video,
) -> Result<Option<Vec<MessageObject>>, AppError> {
    let bytes = app_context
        .line_client
        .get_message_content(&message.id)
        .await
        .map_err(AppError::LineBotSdkError)?;
    app_context
        .destination
        .save_file(&bytes, &format!("{}.mp4", message.id))?;
    Ok(Some(vec![TextMessage::builder()
        .text(&format!(
            "動画を受け取りました！\nメッセージID: {}",
            message.id
        ))
        .build()
        .into()]))
}
