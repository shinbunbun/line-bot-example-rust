use line_bot_sdk::{
    error::AppError,
    models::{
        message::text::TextMessage,
        message::{EachMessageFields, MessageObject},
        webhook_event::Video,
    },
};

pub fn handler(message: &Video) -> Result<Vec<MessageObject>, AppError> {
    println!("{:?}", message);
    Ok(vec![{
        MessageObject {
            quick_reply: None,
            sender: None,
            message: EachMessageFields::Text(TextMessage {
                text: "動画を受け取りました！".to_string(),
                type_field: "text".to_string(),
                emojis: None,
            }),
        }
    }])
}
