use line_bot_sdk::{
    error::AppError,
    models::{
        message::{quick_reply::QuickReply, MessageObject},
        message::{text::TextMessage, CommonFields, quick_reply::Item, stamp::StampMessage, image::ImageMessage, audio::AudioMessage, video::VideoMessage, location::LocationMessage, imagemap::{ImagemapMessage, BaseSize, self, URIAction, Area}},
        webhook_event::Text, action::{Actions, CameraAction, CameraRollAction, LocationAction},
    },
};

pub fn text_event(message: &Text) -> Result<Vec<MessageObject>, AppError> {
    let message = message;
    let messages = match message.text.as_str() {
        "こんにちは" => vec![MessageObject::Text(TextMessage::new(
            "Hello, World!".to_string(),
        ))],
        "複数メッセージ" => vec![
            MessageObject::Text(TextMessage::new("Hello, user".to_string())),
            MessageObject::Text(TextMessage::new("May I help you?".to_string())),
        ],
        "クイックリプライ" => vec![
            MessageObject::Text(TextMessage::new("クイックリプライ（以下のアクションはクイックリプライ専用で、他のメッセージタイプでは使用できません）".to_string()).with_quick_reply(QuickReply{ items: vec![
                Item::new(Actions::CameraAction(CameraAction::new("カメラを開く".to_string()))),
                Item::new(Actions::CameraRollAction(CameraRollAction::new("カメラロールを開く".to_string()))), 
                Item::new(Actions::LocationAction(LocationAction::new("位置情報画面を開く".to_string()))),
            ]})) 
        ],
        "スタンプメッセージ" => vec![
            MessageObject::Stamp(StampMessage::new("446".to_string(), "1988".to_string())),
        ],
        "画像メッセージ" => vec![
            MessageObject::Image(ImageMessage::new("https://shinbunbun.info/images/photos/7.jpeg".to_string(), "https://shinbunbun.info/images/photos/7.jpeg".to_string()))
        ],
        "音声メッセージ" => vec![
            MessageObject::Audio(AudioMessage::new("https://github.com/shinbunbun/aizuhack-bot/blob/master/media/demo.m4a?raw=true".to_string(), 6000))
        ],
        "動画メッセージ" => vec![
            MessageObject::Video(VideoMessage::new("https://github.com/shinbunbun/aizuhack-bot/blob/master/media/demo.mp4?raw=true".to_string(), "https://raw.githubusercontent.com/shinbunbun/aizuhack-bot/master/media/thumbnail.jpg?raw=true".to_string()))
        ],
        "位置情報メッセージ" => vec![
            MessageObject::Location(LocationMessage::new("my location".to_string(), "〒160-0004 東京都新宿区四谷一丁目6番1号".to_string(), 35.687574 ,139.72922))
        ],
        "イメージマップメッセージ" => vec![
            MessageObject::Imagemap(ImagemapMessage::new("https://youkan-storage.s3.ap-northeast-1.amazonaws.com/ubic_bunbun".to_string(), "This is an imagemap".to_string(), BaseSize{width: 1040, height: 597}, imagemap::Action::URIAction(URIAction::new("https://www.u-aizu.ac.jp/intro/faculty/ubic/".to_string(), Area{ x: 26, y: 113, width: 525, height: 170 })) ))
        ],
        _ => vec![{
            MessageObject::Text(TextMessage::new(format!(
                "受け取ったメッセージ: {}\nそのメッセージの返信には対応してません...",
                message.text
            )))
        }],
    };
    Ok(messages)
}
