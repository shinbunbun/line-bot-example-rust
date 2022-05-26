use line_bot_sdk::{
    error::AppError,
    models::{
        message::MessageObject,
        message::{stamp::StampMessage, text::TextMessage},
        webhook_event::Sticker,
    },
};

pub fn handler(message: &Sticker) -> Result<Option<Vec<MessageObject>>, AppError> {
    if message.sticker_id == "1988" {
        /* Ok(Some(vec![MessageObject::Stamp(StampMessage::new(
            "446".to_string(),
            "1989".to_string(),
        ))])) */
        Ok(Some(vec![StampMessage::builder()
            .package_id("446")
            .sticker_id("1989")
            .build()
            .into()]))
    } else {
        /* Ok(Some(vec![MessageObject::Text(TextMessage::new(format!(
            "受け取ったstickerId: {}\nそのスタンプの返信には対応してません...",
            message.sticker_id
        )))])) */
        Ok(Some(vec![TextMessage::builder()
            .text(&format!(
                "受け取ったstickerId: {}\nそのスタンプの返信には対応してません...",
                message.sticker_id
            ))
            .build()
            .into()]))
    }
}
