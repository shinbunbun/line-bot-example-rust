use line_bot_sdk::{
    client::Client,
    error::AppError,
    models::{
        action::{
             CameraAction, CameraRollAction, DatetimePickerAction, LocationAction,
            MessageAction, PostbackAction, URIAction,
        },
        message::{
            audio::AudioMessage,
            image::ImageMessage,
            imagemap::ImagemapMessage,
            location::LocationMessage,
            stamp::StampMessage,
            text::TextMessage,
            video::VideoMessage,
        },
        message::{
            flex::FlexMessage,
            imagemap::ImagemapURIAction,
            quick_reply::{QuickReply, QuickReplyItem},
            template::{
                buttons::ButtonsTemplate,
                carousel::{self, CarouselTemplate},
                confirm::ConfirmTemplate,
                image_carousel::{self, ImageCarouselTemplate},
                TemplateMessage,
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
    let messages: Vec<MessageObject> = match message.text.as_str() {
        "こんにちは" => vec![
            // MessageObject::Text(TextMessage::new("Hello, World!".to_string()))
            /* MessageBuilder::new()
            .text_message("Hello, World")
            .build() */
            TextMessage::builder().text("Hello, World").build().into(),
        ],
        "複数メッセージ" => vec![
            // MessageObject::Text(TextMessage::new("Hello, user".to_string())),
            // MessageObject::Text(TextMessage::new("May I help you?".to_string())),
            /* MessageBuilder::new()
            .text_message("Hello, user")
            .build(),
            MessageBuilder::new()
            .text_message("May I help you?")
            .build() */
            TextMessage::builder().text("Hello, user").build().into(),
            TextMessage::builder().text("May I help you?").build().into(),
        ],
        "クイックリプライ" => vec![
            /* MessageObject::Text(TextMessage::new("クイックリプライ（以下のアクションはクイックリプライ専用で、他のメッセージタイプでは使用できません）".to_string()).with_quick_reply(QuickReply{ items: vec![
                Item::new(Actions::CameraAction(CameraAction::new("カメラを開く".to_string()))),
                Item::new(Actions::CameraRollAction(CameraRollAction::new("カメラロールを開く".to_string()))), 
                Item::new(Actions::LocationAction(LocationAction::new("位置情報画面を開く".to_string()))),
            ]})) */
            /* MessageBuilder::new()
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
            .build() */
            TextMessage::builder()
            .text("クイックリプライ（以下のアクションはクイックリプライ専用で、他のメッセージタイプでは使用できません）")
            .quick_reply(
                QuickReply::builder()
                .items(
                    vec![
                        QuickReplyItem::builder()
                        .action(CameraAction::builder().label("カメラを開く").build().into())
                        .build(),
                        QuickReplyItem::builder()
                        .action(CameraRollAction::builder().label("カメラロールを開く").build().into())
                        .build(),
                        QuickReplyItem::builder()
                        .action(LocationAction::builder().label("位置情報画面を開く").build().into())
                        .build(),
                    ],
                ).build()
            ).build().into(),

        ],
        "スタンプメッセージ" => vec![
            // MessageObject::Stamp(StampMessage::new("446".to_string(), "1988".to_string())),
            /* MessageBuilder::new()
            .stamp_message("446", "1988")
            .build() */
            StampMessage::builder()
            .package_id("446")
            .sticker_id("1988")
            .build()
            .into(),
        ],
        "画像メッセージ" => vec![
            // MessageObject::Image(ImageMessage::new("https://shinbunbun.info/images/photos/7.jpeg".to_string(), "https://shinbunbun.info/images/photos/7.jpeg".to_string()))
            /* MessageBuilder::new()
            .image_message("https://shinbunbun.info/images/photos/7.jpeg", "https://shinbunbun.info/images/photos/7.jpeg")
            .build() */
            ImageMessage::builder()
            .original_content_url("https://shinbunbun.info/images/photos/7.jpeg")
            .preview_image_url("https://shinbunbun.info/images/photos/7.jpeg")
            .build()
            .into()
        ],
        "音声メッセージ" => vec![
            // MessageObject::Audio(AudioMessage::new("https://github.com/shinbunbun/aizuhack-bot/blob/master/media/demo.m4a?raw=true".to_string(), 6000))
            /* MessageBuilder::new()
            .audio_message("https://github.com/shinbunbun/aizuhack-bot/blob/master/media/demo.m4a?raw=true", 6000)
            .build() */
            AudioMessage::builder()
            .original_content_url("https://github.com/shinbunbun/aizuhack-bot/blob/master/media/demo.m4a?raw=true")
            .duration(6000)
            .build()
            .into()
        ],
        "動画メッセージ" => vec![
            // MessageObject::Video(VideoMessage::new("https://github.com/shinbunbun/aizuhack-bot/blob/master/media/demo.mp4?raw=true".to_string(), "https://raw.githubusercontent.com/shinbunbun/aizuhack-bot/master/media/thumbnail.jpg?raw=true".to_string()))
            /* MessageBuilder::new()
            .video_message("https://github.com/shinbunbun/aizuhack-bot/blob/master/media/demo.mp4?raw=true", "https://raw.githubusercontent.com/shinbunbun/aizuhack-bot/master/media/thumbnail.jpg?raw=true")
            .build() */
            VideoMessage::builder()
            .original_content_url("https://github.com/shinbunbun/aizuhack-bot/blob/master/media/demo.mp4?raw=true")
            .preview_image_url("https://raw.githubusercontent.com/shinbunbun/aizuhack-bot/master/media/thumbnail.jpg?raw=true")
            .build()
            .into()
        ],
        "位置情報メッセージ" => vec![
            // MessageObject::Location(LocationMessage::new("my location".to_string(), "〒160-0004 東京都新宿区四谷一丁目6番1号".to_string(), 35.687574 ,139.72922))
            /* MessageBuilder::new()
            .location_message("my location", "〒160-0004 東京都新宿区四谷一丁目6番1号", 35.687574, 139.72922)
            .build() */
            LocationMessage::builder()
            .title("my location")
            .address("〒160-0004 東京都新宿区四谷一丁目6番1号")
            .latitude(35.687574)
            .longitude(139.72922)
            .build()
            .into()
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
            /* MessageBuilder::new()
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
            MessageBuilder::new().text_message("「UBIC」や「しんぶんぶん」のところをTAPしてみよう!").build(), */
            ImagemapMessage::builder()
            .base_url("https://youkan-storage.s3.ap-northeast-1.amazonaws.com/ubic_bunbun")
            .alt_text("This is an imagemap")
            .base_size(1040, 597)
            .actions(vec![
                ImagemapURIAction::builder()
                .link_uri("https://www.u-aizu.ac.jp/intro/faculty/ubic/")
                .area(26, 113, 525, 170)
                .build()
                .into(),
                ImagemapURIAction::builder()
                .link_uri("https://shinbunbun.info/about/")
                .area(33, 331, 780, 177)
                .build()
                .into(),
                ImagemapURIAction::builder()
                .link_uri("https://www.u-aizu.ac.jp/")
                .area(939, 484, 94, 105)
                .build()
                .into(),
            ])
            .build()
            .into(),
        ],
        "ボタンテンプレート" => vec![
            /* MessageObject::Template(TemplateMessage::new(
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
            )))) */
            TemplateMessage::builder()
            .alt_text("ボタンテンプレート")
            .template(
                ButtonsTemplate::builder()
                .text("ボタンだお")
                .default_action(
                    URIAction::builder()
                    .uri("https://shinbunbun.info/images/photos/")
                    .build()
                    .into()
                )
                .actions(vec![
                    PostbackAction::builder()
                    .label("ポストバックアクション")
                    .data("button-postback")
                    .build()
                    .into(),
                    MessageAction::builder()
                    .label("メッセージアクション")
                    .text("button-message")
                    .build()
                    .into(),
                    URIAction::builder()
                    .label("URIアクション")
                    .uri("https://shinbunbun.info/")
                    .build()
                    .into(),
                    DatetimePickerAction::builder()
                    .data("button-date")
                    .mode("datetime")
                    .label("日時選択アクション")
                    .initial("2021-06-01t00:00")
                    .max("2022-12-31t23:59")
                    .min("2021-06-01t00:00")
                    .build()
                    .into(),
                ])
                .build()
                .into(),
            )
            .build()
            .into(),
        ], 
        "確認テンプレート" => vec![
           /*  MessageObject::Template(TemplateMessage::new(
            "確認テンプレート".to_string(),
            Template::Confirm(ConfirmTemplate::new(
                "確認テンプレート".to_string(),
                vec![
                    Actions::MessageAction(MessageAction::new(Some("はい".to_string()), "yes".to_string())),
                    Actions::MessageAction(MessageAction::new(Some("いいえ".to_string()), "no".to_string())),
                ]
            ))
            )) */
            TemplateMessage::builder()
            .alt_text("確認テンプレート")
            .template(
                ConfirmTemplate::builder()
                .text("確認テンプレート")
                .actions(vec![
                    MessageAction::builder()
                    .label("はい")
                    .text("yes")
                    .build()
                    .into(),
                    MessageAction::builder()
                    .label("いいえ")
                    .text("no")
                    .build()
                    .into(),
                ])
                .build()
                .into(),
            )
            .build()
            .into(),
        ],
        "カルーセルテンプレート" => vec![
            /* MessageObject::Template(TemplateMessage::new(
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
            )) */
            TemplateMessage::builder()
            .alt_text("カルーセルテンプレート")
            .template(
                CarouselTemplate::builder()
                .columns(vec![
                    carousel::Column::builder()
                    .text("説明1")
                    .actions(vec![
                        PostbackAction::builder()
                        .label("ポストバック")
                        .data("postback-carousel-1")
                        .build()
                        .into(),
                        URIAction::builder()
                        .label("URIアクション")
                        .uri("https://shinbunbun.info/")
                        .build()
                        .into(),
                    ])
                    .default_action(
                        URIAction::builder()
                        .uri("https://shinbunbun.info/images/photos/")
                        .build()
                        .into()
                    )
                    .thumbnail_image_url("https://shinbunbun.info/images/photos/7.jpeg")
                    .image_background_color("#FFFFFF")
                    .title("タイトル1")
                    .build(),
                ])
                .build()
                .into()
            )
            .build()
            .into(),
        ],
        "画像カルーセルテンプレート"=> vec![
            /* MessageObject::Template(TemplateMessage::new(
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
            )), */
            TemplateMessage::builder()
            .alt_text("画像カルーセルテンプレート")
            .template(
                ImageCarouselTemplate::builder()
                .columns(vec![
                    image_carousel::Column::builder()
                    .image_url("https://shinbunbun.info/images/photos/4.jpeg")
                    .action(
                        PostbackAction::builder()
                        .label("ポストバック")
                        .data("image-carousel-1")
                        .build()
                        .into(),
                    )
                    .build(),
                    image_carousel::Column::builder()
                    .image_url("https://shinbunbun.info/images/photos/5.jpeg")
                    .action(
                        MessageAction::builder()
                        .label("メッセージ")
                        .text("text")
                        .build()
                        .into(),
                    )
                    .build(),
                    image_carousel::Column::builder()
                    .image_url("https://shinbunbun.info/images/photos/7.jpeg")
                    .action(
                        URIAction::builder()
                        .label("URIアクション")
                        .uri("https://shinbunbun.info/")
                        .build()
                        .into(),
                    )
                    .build(),
                ])
                .build()
                .into()
            )
            .build()
            .into(),
        ],
        "Flex Message" => {
            let flex_message = FlexMessage::from_json(r##"{"type":"flex","altText":"Flex Message","contents":{"type":"bubble","header":{"type":"box","layout":"vertical","contents":[{"type":"text","text":"Flex Message","color":"#FFFFFF","weight":"bold"}]},"hero":{"type":"image","url":"https://pbs.twimg.com/profile_images/1236928986212478976/wDa51i9T_400x400.jpg","size":"xl"},"body":{"type":"box","layout":"vertical","contents":[{"type":"text","text":"しんぶんぶん","size":"xl","weight":"bold","align":"center"},{"type":"text","text":"会津大学学部一年","align":"center"},{"type":"separator","margin":"md"},{"type":"box","layout":"vertical","contents":[{"type":"button","action":{"type":"uri","label":"ホームページ","uri":"https://shinbunbun.info/"},"style":"primary","offsetBottom":"10px"},{"type":"button","action":{"type":"uri","label":"Twitter","uri":"https://twitter.com/shinbunbun_"},"style":"primary","color":"#1DA1F2"}],"paddingTop":"10px"}]},"styles":{"header":{"backgroundColor":"#008282"}}}}"##)?;
            vec![MessageObject::Flex(flex_message)]
        }
        "プロフィール"=>{
            let profile = client.get_profile(&event.source.user_id.as_ref().ok_or_else(|| AppError::BadRequest("userId not found".to_string()))?).await?;
           /*  vec![
                MessageObject::Text(TextMessage::new(format!("あなたの名前: {}\nユーザーID: {}\nプロフィール画像のURL: {}\nステータスメッセージ: {}", profile.display_name, profile.user_id, profile.picture_url, profile.status_message))),
            ] */
            vec![
                TextMessage::builder()
                .text(&format!("あなたの名前: {}\nユーザーID: {}\nプロフィール画像のURL: {}\nステータスメッセージ: {}", profile.display_name, profile.user_id, profile.picture_url, profile.status_message))
                .build()
                .into(),
            ]
        },
        "ここはどこ"=>{
            if event.source.type_field=="user"{
                /* vec![MessageObject::Text(TextMessage::new("ここは個チャだよ!".to_string()))] */
                vec![
                    TextMessage::builder()
                    .text("ここは個チャだよ!")
                    .build()
                    .into(),
                ]
            } else if event.source.type_field=="group"{
                // vec![MessageObject::Text(TextMessage::new("ここはグループだよ!".to_string()))]
                vec![
                    TextMessage::builder()
                    .text("ここはグループだよ!")
                    .build()
                    .into(),
                ]
            } else {
                // vec![MessageObject::Text(TextMessage::new("非対応のソースです".to_string()))]
                vec![
                    TextMessage::builder()
                    .text("非対応のソースです")
                    .build()
                    .into(),
                ]
            }
        }
        _ => vec![
            /* MessageObject::Text(TextMessage::new(format!(
                "受け取ったメッセージ: {}\nそのメッセージの返信には対応してません...",
                message.text
            ))) */
                TextMessage::builder()
                .text(&format!("受け取ったメッセージ: {}\nそのメッセージの返信には対応してません...", message.text))
                .build()
                .into(),
        ],
    };
    Ok(Some(messages))
}
