use std::{collections::HashMap, hash::Hash};

use log::info;
use serde::{
    de::{self, value::MapAccessDeserializer, IntoDeserializer, VariantAccess, Visitor},
    Deserialize,
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub destination: String,
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "type")]
    pub type_field: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub webhook_event_id: String,
    pub delivery_context: DeliveryContext,
    pub message: Option<Message>,
    pub reply_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryContext {
    pub is_redelivery: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
// TODO: VideoとImageの順番を入れ替えるとVideoがImageとしてDesirializeされる問題を調査
pub enum Message {
    Text(Text),
    Video(Video),
    Image(Image),
    Audio(Audio),
}

/* impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FieldVisitor;
        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Message;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("`text`, `video`, or `image`")
            }

            /* fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Deserialize::deserialize(value.into_deserializer()).map(Message::Text)
            } */

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                /* while let Some(x) = map.next_entry()? {
                    println!("{:?}", x);
                    return Deserialize::deserialize(MapAccessDeserializer::new(map))
                        .map(Message::Text);
                } */
                /* while true {
                    if(key.0 == "type") {
                        match key.1 {
                            "text" => {


                        }
                    }
                } */
                /* while let Some(x) = map.next_entry()? {
                    println!("{:?}", x);
                }; */
                let mut message_map = HashMap::<String, String>::new();
                let mut message_type: String;
                while let Some((key, value)) = map.next_entry::<String, String>()? {
                    println!("{}, {}", key, value);
                    message_map.insert(key.clone(), value.clone());
                    if key != "type" {
                        continue;
                    }
                    match value.as_str() {
                        "text" => {
                            message_type = value;
                        }
                        _ => {
                            return Err(serde::de::Error::custom("unexpected message type"));
                        }
                    }
                }
                println!("{:#?}", message_map);
                Deserialize::deserialize(MapAccessDeserializer::new(map)).map(Message::Text)
            }
        }
        deserializer.deserialize_any(FieldVisitor)
    }
} */

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub id: String,
    // #[serde(rename = "type")]
    // pub type_field: String,
    pub text: String,
    pub emojis: Option<Vec<Emoji>>,
    pub mention: Option<Mention>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: String,
    pub content_provider: ContentProvider,
    pub image_set: Option<ImageSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentProvider {
    #[serde(rename = "type")]
    pub type_field: String,
    pub original_content_url: Option<String>,
    pub preview_image_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageSet {
    pub id: String,
    pub index: i64,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub index: i64,
    pub length: i64,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub mentionees: Vec<Mentionee>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mentionee {
    pub index: i64,
    pub length: i64,
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename = "type")]
    pub type_field: String,
    pub user_id: String,
    pub group_id: Option<String>,
    pub room_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    pub duration: i64,
    pub content_provider: ContentProvider,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub id: String,
    pub duration: i64,
    pub content_provider: ContentProvider,
}
