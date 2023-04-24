use line_bot_sdk::models::{
    message::text::TextMessage, message::MessageObject, webhook_event::File,
};

use crate::{app_context::AppContext, error::AppError};

pub async fn handler(
    app_context: &AppContext,
    message: &File,
) -> Result<Option<Vec<MessageObject>>, AppError> {
    let bytes = app_context
        .line_client
        .get_message_content(&message.id)
        .await
        .map_err(AppError::LineBotSdkError)?;
    app_context.destination.save_file(&bytes, &message.id)?;
    Ok(Some(vec![TextMessage::builder()
        .text(&format!(
            "ファイルを受け取りました！\nメッセージID: {}",
            message.id
        ))
        .build()
        .into()]))
}
