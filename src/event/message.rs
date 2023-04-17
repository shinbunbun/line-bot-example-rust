use line_bot_sdk::models::{
    message::MessageObject,
    webhook_event::{Event, Message},
};
use log::info;

use crate::{app_context::AppContext, error::AppError};

pub mod audio;
pub mod file;
pub mod image;
pub mod location;
pub mod sticker;
pub mod text;
pub mod video;

pub async fn index(
    app_context: &AppContext,
    event: &Event,
) -> Result<Option<Vec<MessageObject>>, AppError> {
    info!("{:?}", event);
    let message = event
        .message
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Message not found".to_string()))?;
    match message {
        Message::Text(text_message) => text::text_event(app_context, event, text_message).await,
        Message::Image(image_message) => image::handler(image_message),
        Message::Video(video_message) => video::handler(video_message),
        Message::Audio(audio_message) => audio::handler(app_context, audio_message).await,
        Message::File(file_message) => file::handler(file_message),
        Message::Location(location_message) => location::handler(location_message),
        Message::Sticker(sticker_message) => sticker::handler(sticker_message),
    }
}
