use line_bot_sdk::{
    error::AppError,
    models::{
        message::{text::TextMessage, EachMessageFields, MessageObject},
        webhook_event::{Event, Message},
    },
};

use super::{audio, image, text, video};

pub fn index(event: &Event) -> Result<Vec<MessageObject>, AppError> {
    let message = event
        .message
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Message not found".to_string()))?;
    match message {
        Message::Text(text_message) => text::text_event(text_message),
        Message::Image(image_message) => image::handler(image_message),
        Message::Video(video_message) => video::handler(video_message),
        Message::Audio(audio_message) => audio::handler(audio_message),
        _ => Ok(vec![{
            MessageObject {
                quick_reply: None,
                sender: None,
                message: EachMessageFields::Text(TextMessage {
                    text: "そのイベントには対応していません...".to_string(),
                    type_field: "text".to_string(),
                    emojis: None,
                }),
            }
        }]),
    }
}
