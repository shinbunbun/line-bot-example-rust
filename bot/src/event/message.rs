use line_bot_sdk::{
    client::Client,
    error::AppError,
    models::{
        message::{text::TextMessage, MessageObject},
        webhook_event::{Event, Message},
    },
};
use log::info;

use super::{audio, image, text, video};

pub async fn index(client: &Client, event: &Event) -> Result<Vec<MessageObject>, AppError> {
    info!("{:?}", event);
    let message = event
        .message
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Message not found".to_string()))?;
    match message {
        Message::Text(text_message) => text::text_event(client, event, text_message).await,
        Message::Image(image_message) => image::handler(image_message),
        Message::Video(video_message) => video::handler(video_message),
        Message::Audio(audio_message) => audio::handler(audio_message),
        _ => Ok(vec![MessageObject::Text(TextMessage::new(
            "そのイベントには対応していません...".to_string(),
        ))]),
    }
}
