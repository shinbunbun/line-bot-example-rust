use line_bot_sdk::{
    error::AppError,
    models::{
        message::text::TextMessage,
        message::{EachMessageFields, MessageObject},
        webhook_event::Image,
    },
};

pub fn handler(message: &Image) -> Result<Vec<MessageObject>, AppError> {
    println!("{:?}", message);
    Ok(vec![{
        MessageObject {
            quick_reply: None,
            sender: None,
            message: EachMessageFields::Text(TextMessage {
                text: "画像を受け取りました！".to_string(),
                type_field: "text".to_string(),
                emojis: None,
            }),
        }
    }])
}
