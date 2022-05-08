use line_bot_sdk::{
    error::AppError,
    models::{
        message::{text::TextMessage, EachMessageFields, MessageObject},
        webhook_event::{Event, Message},
    },
};

pub fn text_event(message: &Message) -> Result<Vec<MessageObject>, AppError> {
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

pub fn index(event: &Event) -> Result<Vec<MessageObject>, AppError> {
    let message = event
        .message
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Message not found".to_string()))?;
    match message.type_field.as_str() {
        "text" => text_event(message),
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
