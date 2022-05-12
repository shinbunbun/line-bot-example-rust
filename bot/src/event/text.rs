use line_bot_sdk::{
    error::AppError,
    models::{
        message::text::TextMessage,
        message::{
            quick_reply::{Item, QuickReply},
            /* EachMessageFields, */ MessageObject,
        },
        webhook_event::Text,
    },
};

pub fn text_event(message: &Text) -> Result<Vec<MessageObject>, AppError> {
    let message = message;
    let messages = match message.text.as_str() {
        "こんにちは" => vec![MessageObject::Text(TextMessage::new(
            "Hello, World!".to_string(),
            None,
        ))],
        "複数メッセージ" => vec![
            MessageObject::Text(TextMessage::new("Hello, user".to_string(), None)),
            MessageObject::Text(TextMessage::new("May I help you?".to_string(), None)),
        ],
        _ => vec![{
            MessageObject::Text(TextMessage::new(
                format!(
                    "受け取ったメッセージ: {}\nそのメッセージの返信には対応してません...",
                    message.text
                ),
                None,
            ))
        }],
    };
    Ok(messages)
}
