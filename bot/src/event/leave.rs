use line_bot_sdk::{error::AppError, models::message::MessageObject};

pub async fn index() -> Result<Option<Vec<MessageObject>>, AppError> {
    println!("botがグループから退出しました");
    Ok(None)
}
