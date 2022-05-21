use line_bot_sdk::{
    error::AppError,
    models::{
        message::MessageObject,
        message::{stamp::StampMessage, text::TextMessage},
        webhook_event::Sticker,
    },
};

pub fn handler(message: &Sticker) -> Result<Vec<MessageObject>, AppError> {
    if message.sticker_id == "1988" {
        Ok(vec![MessageObject::Stamp(StampMessage::new(
            "446".to_string(),
            "1989".to_string(),
        ))])
    } else {
        Ok(vec![MessageObject::Text(TextMessage::new(format!(
            "受け取ったstickerId: {}\nそのスタンプの返信には対応してません...",
            message.sticker_id
        )))])
    }
}
