use line_bot_sdk::models::message::MessageObject;

use crate::error::AppError;

pub async fn index() -> Result<Option<Vec<MessageObject>>, AppError> {
    println!("botがグループから退出しました");
    Ok(None)
}
