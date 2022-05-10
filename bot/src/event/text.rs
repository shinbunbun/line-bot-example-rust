use line_bot_sdk::{
    error::AppError,
    models::{
        message::text::TextMessage,
        message::{EachMessageFields, MessageObject},
        webhook_event::Text,
    },
};

pub fn text_event(message: &Text) -> Result<Vec<MessageObject>, AppError> {
    let message = message;
    match message.text.as_str() {
        "こんにちは" => Ok(vec![{
            MessageObject {
                quick_reply: None,
                sender: None,
                message: EachMessageFields::Text(TextMessage {
                    text: "Hello, world".to_string(),
                    type_field: "text".to_string(),
                    emojis: None,
                }),
            }
        }]),
        _ => Ok(vec![{
            MessageObject {
                quick_reply: None,
                sender: None,
                message: EachMessageFields::Text(TextMessage {
                    text: format!(
                        "受け取ったメッセージ: {}\nそのメッセージの返信には対応してません...",
                        message.text
                    ),
                    type_field: "text".to_string(),
                    emojis: None,
                }),
            }
        }]),
    }
}
