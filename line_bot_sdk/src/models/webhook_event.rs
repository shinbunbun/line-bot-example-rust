use serde::Deserialize;

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
    pub unsend: Option<Unsend>,
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
    File(File),
    Location(Location),
    Stamp(Stamp),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub id: String,
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: String,
    pub file_name: String,
    pub file_size: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stamp {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub sticker_resource_type: ResourceType,
    pub keywords: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ResourceType {
    STATIC,
    ANIMATION,
    SOUND,
    ANIMATION_SOUND,
    POPUP,
    POPUP_SOUND,
    CUSTOM,
    MESSAGE,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Unsend {
    message_id: String,
}
