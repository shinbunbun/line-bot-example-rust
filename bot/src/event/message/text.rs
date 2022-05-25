use line_bot_sdk::{
    builder::{
        action::ActionBuilder,
        imagemap::ImageMapURIActionBuilder,
        message::MessageBuilder,
        quick_reply::{QuickReplyBuilder, QuickReplyItemBuilder},
    },
    client::Client,
    error::AppError,
    models::{
        action::{
            Actions, CameraAction, CameraRollAction, DatetimePickerAction, LocationAction,
            MessageAction, PostbackAction, URIAction,
        },
        message::{
            audio::AudioMessage,
            image::ImageMessage,
            imagemap::{self, Area, BaseSize, ImagemapMessage},
            location::LocationMessage,
            quick_reply::Item,
            stamp::StampMessage,
            text::TextMessage,
            video::VideoMessage,
        },
        message::{
            flex::FlexMessage,
            quick_reply::QuickReply,
            template::{
                buttons::ButtonsTemplate,
                carousel::{self, CarouselTemplate},
                confirm::ConfirmTemplate,
                image_carousel::{self, ImageCarouselTemplate},
                Template, TemplateMessage,
            },
            Message, MessageObject,
        },
        webhook_event::{Event, Text},
    },
};

pub async fn text_event(
    client: &Client,
    event: &Event,
    message: &Text,
) -> Result<Option<Vec<MessageObject>>, AppError> {
    let messages = match message.text.as_str() {
        "こんにちは" => vec![
            // MessageObject::Text(TextMessage::new("Hello, World!".to_string()))
            MessageBuilder::new()
            .text_message("Hello, World")
            .build()
        ],
        "複数メッセージ" => vec![
            // MessageObject::Text(TextMessage::new("Hello, user".to_string())),
            // MessageObject::Text(TextMessage::new("May I help you?".to_string())),
            MessageBuilder::new()
            .text_message("Hello, user")
            .build(),
            MessageBuilder::new()
            .text_message("May I help you?")
            .build()
        ],
        "クイックリプライ" => vec![
            /* MessageObject::Text(TextMessage::new("クイックリプライ（以下のアクションはクイックリプライ専用で、他のメッセージタイプでは使用できません）".to_string()).with_quick_reply(QuickReply{ items: vec![
                Item::new(Actions::CameraAction(CameraAction::new("カメラを開く".to_string()))),
                Item::new(Actions::CameraRollAction(CameraRollAction::new("カメラロールを開く".to_string()))), 
                Item::new(Actions::LocationAction(LocationAction::new("位置情報画面を開く".to_string()))),
            ]})) */
            MessageBuilder::new()
            .text_message("クイックリプライ（以下のアクションはクイックリプライ専用で、他のメッセージタイプでは使用できません）")
            .with_quick_reply(
                QuickReplyBuilder::new()
                .add_item(
                    QuickReplyItemBuilder::new()
                    .action(ActionBuilder::new().camera_action("カメラを開く").build())
                    .build(),
                )
                .add_item(
                    QuickReplyItemBuilder::new()
                    .action(ActionBuilder::new().camera_roll_action("カメラロールを開く").build())
                    .build(),
                )
                .add_item(
                    QuickReplyItemBuilder::new()
                    .action(ActionBuilder::new().location_action("位置情報画面を開く").build())
                    .build(),
                )
                .build(),
            )
            .build()
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
            /* MessageObject::Imagemap(ImagemapMessage::new(
                "https://youkan-storage.s3.ap-northeast-1.amazonaws.com/ubic_bunbun".to_string(), 
                "This is an imagemap".to_string(), 
                BaseSize{width: 1040, height: 597},
                vec![
                    imagemap::Action::URIAction(imagemap::URIAction::new("https://www.u-aizu.ac.jp/intro/faculty/ubic/".to_string(),Area{ x: 26, y: 113, width: 525, height: 170 })),
                    imagemap::Action::URIAction(imagemap::URIAction::new("https://shinbunbun.info/about/".to_string(), Area{x:33, y:331, width: 780, height:177})),
                    imagemap::Action::URIAction(imagemap::URIAction::new("https://www.u-aizu.ac.jp/".to_string(), Area{x:939, y:484, width: 94, height:105})),
                ],
            )), */
            MessageBuilder::new()
            .imagemap_message(
                "https://youkan-storage.s3.ap-northeast-1.amazonaws.com/ubic_bunbun",
                "This is an imagemap",
            )
            .base_size(1040, 597)
            .add_action(
                ImageMapURIActionBuilder::new(
                    "https://www.u-aizu.ac.jp/intro/faculty/ubic/",
                    Area::new(26, 113, 525, 170),
                )
                .build(),
            )
            .add_action(
                ImageMapURIActionBuilder::new(
                    "https://shinbunbun.info/about/",
                    Area::new(33, 331, 780, 177),
                )
                .build(),
            )
            .add_action(
                ImageMapURIActionBuilder::new(
                    "https://www.u-aizu.ac.jp/",
                    Area::new(939, 484, 94, 105),
                )
                .build(),
            )
            .build(),
            MessageObject::Text(TextMessage::new("「UBIC」や「しんぶんぶん」のところをTAPしてみよう!".to_string()))
        ],
        "ボタンテンプレート" => vec![MessageObject::Template(TemplateMessage::new(
            "ボタンテンプレート".to_string(),
            Template::Buttons(ButtonsTemplate::new(
                "ボタンだお".to_string(),
                Actions::URIAction(URIAction::new(Some("View detail".to_string()), "https://shinbunbun.info/images/photos/".to_string())),
                vec![
                    Actions::PostbackAction(PostbackAction::new("ポストバックアクション".to_string(), "button-postback".to_string())),
                    Actions::MessageAction(MessageAction::new(Some("メッセージアクション".to_string()), "button-message".to_string())),
                    Actions::URIAction(URIAction::new(Some("URIアクション".to_string()), "https://shinbunbun.info/".to_string())),
                    Actions::DatetimePickerAction(DatetimePickerAction::new("button-date".to_string(), "datetime".to_string(), Some("日時選択アクション".to_string()))
                    .with_initial("2021-06-01t00:00".to_string())
                    .with_max("2022-12-31t23:59".to_string())
                    .with_min("2021-06-01t00:00".to_string()))
                ],
            ))))],
        "確認テンプレート" => vec![MessageObject::Template(TemplateMessage::new(
            "確認テンプレート".to_string(),
            Template::Confirm(ConfirmTemplate::new(
                "確認テンプレート".to_string(),
                vec![
                    Actions::MessageAction(MessageAction::new(Some("はい".to_string()), "yes".to_string())),
                    Actions::MessageAction(MessageAction::new(Some("いいえ".to_string()), "no".to_string())),
                ]
            ))
        ))],
        "カルーセルテンプレート" => vec![MessageObject::Template(TemplateMessage::new(
            "カルーセルテンプレート".to_string(),
            Template::Carousel(CarouselTemplate::new(vec![
                carousel::Column::new(
                    "説明1".to_string(),
                    vec![
                        Actions::PostbackAction(PostbackAction::new("ポストバック".to_string(), "postback-carousel-1".to_string())),
                        Actions::URIAction(URIAction::new(Some("URIアクション".to_string()), "https://shinbunbun.info/".to_string())),
                    ]
                )
                .with_default_action(Actions::URIAction(URIAction::new(Some("View detail".to_string()), "https://shinbunbun.info/".to_string())))
                .with_thumbnail_image_url("https://shinbunbun.info/images/photos/7.jpeg".to_string())
                .with_image_background_color("#FFFFFF".to_string())
                .with_title("タイトル1".to_string()),
                carousel::Column::new(
                    "説明2".to_string(),
                    vec![
                        Actions::PostbackAction(PostbackAction::new("ポストバック".to_string(), "postback-carousel-2".to_string())),
                        Actions::URIAction(URIAction::new(Some("URIアクション".to_string()), "https://shinbunbun.info/".to_string())),
                    ]
                )
                .with_default_action(Actions::URIAction(URIAction::new(Some("View detail".to_string()), "https://shinbunbun.info/".to_string())))
                .with_thumbnail_image_url("https://shinbunbun.info/images/photos/10.jpeg".to_string())
                .with_image_background_color("#FFFFFF".to_string())
                .with_title("タイトル2".to_string()),

            ])
            .with_image_aspect_ratio("rectangle".to_string())
            .with_image_size("cover".to_string())
            )
        ))],
        "画像カルーセルテンプレート"=> vec![
            MessageObject::Template(TemplateMessage::new(
                "画像カルーセルテンプレート".to_string(),
                Template::ImageCarousel(ImageCarouselTemplate::new(
                    vec![
                        image_carousel::Column::new(
                            "https://shinbunbun.info/images/photos/4.jpeg".to_string(),
                            Actions::PostbackAction(PostbackAction::new("ポストバック".to_string(), "image-carousel-1".to_string()))
                        ),
                        image_carousel::Column::new(
                            "https://shinbunbun.info/images/photos/5.jpeg".to_string(),
                            Actions::MessageAction(MessageAction::new(Some("メッセージ".to_string()), "text".to_string()))
                        ),
                        image_carousel::Column::new(
                            "https://shinbunbun.info/images/photos/7.jpeg".to_string(),
                            Actions::URIAction(URIAction::new(Some("URIアクション".to_string()), "https://shinbunbun.info/".to_string()))
                        )
                    ]
                ))
            )),
        ],
        "Flex Message" => {
            let flex_message = FlexMessage::from_json(r##"{"type":"flex","altText":"Flex Message","contents":{"type":"bubble","header":{"type":"box","layout":"vertical","contents":[{"type":"text","text":"Flex Message","color":"#FFFFFF","weight":"bold"}]},"hero":{"type":"image","url":"https://pbs.twimg.com/profile_images/1236928986212478976/wDa51i9T_400x400.jpg","size":"xl"},"body":{"type":"box","layout":"vertical","contents":[{"type":"text","text":"しんぶんぶん","size":"xl","weight":"bold","align":"center"},{"type":"text","text":"会津大学学部一年","align":"center"},{"type":"separator","margin":"md"},{"type":"box","layout":"vertical","contents":[{"type":"button","action":{"type":"uri","label":"ホームページ","uri":"https://shinbunbun.info/"},"style":"primary","offsetBottom":"10px"},{"type":"button","action":{"type":"uri","label":"Twitter","uri":"https://twitter.com/shinbunbun_"},"style":"primary","color":"#1DA1F2"}],"paddingTop":"10px"}]},"styles":{"header":{"backgroundColor":"#008282"}}}}"##)?;
            vec![MessageObject::Flex(flex_message)]
        }
        "プロフィール"=>{
            let profile = client.get_profile(&event.source.user_id.as_ref().ok_or_else(|| AppError::BadRequest("userId not found".to_string()))?).await?;
            vec![
                MessageObject::Text(TextMessage::new(format!("あなたの名前: {}\nユーザーID: {}\nプロフィール画像のURL: {}\nステータスメッセージ: {}", profile.display_name, profile.user_id, profile.picture_url, profile.status_message))),
            ]
        },
        "ここはどこ"=>{
            if event.source.type_field=="user"{
                vec![MessageObject::Text(TextMessage::new("ここは個チャだよ!".to_string()))]
            } else if event.source.type_field=="group"{
                vec![MessageObject::Text(TextMessage::new("ここはグループだよ!".to_string()))]
            } else {
                vec![MessageObject::Text(TextMessage::new("非対応のソースです".to_string()))]
            }
        }
        _ => vec![{
            MessageObject::Text(TextMessage::new(format!(
                "受け取ったメッセージ: {}\nそのメッセージの返信には対応してません...",
                message.text
            )))
        }],
    };
    Ok(Some(messages))
}
