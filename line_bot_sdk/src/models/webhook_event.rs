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
    pub joined: Option<Join>,
    pub left: Option<Left>,
    pub postback: Option<Postback>,
    pub video_play_complete: Option<VideoPlayComplete>,
    pub beacon: Option<Beacon>,
    pub link: Option<Link>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryContext {
    pub is_redelivery: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Message {
    Text(Text),
    Video(Video),
    Image(Image),
    Audio(Audio),
    File(File),
    Location(Location),
    Sticker(Sticker),
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
    pub user_id: Option<String>,
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
    pub title: Option<String>,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sticker {
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
    #[allow(non_camel_case_types)]
    ANIMATION_SOUND,
    POPUP,
    #[allow(non_camel_case_types)]
    POPUP_SOUND,
    CUSTOM,
    MESSAGE,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unsend {
    pub message_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Join {
    pub members: Vec<Source>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Left {
    pub members: Vec<Source>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Postback {
    pub data: String,
    pub params: Option<Params>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub date: Option<String>,
    pub time: Option<String>,
    pub datetime: Option<String>,
    pub new_richmenu_alias_id: Option<String>,
    pub status: Option<Status>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    SUCCESS,
    #[allow(non_camel_case_types)]
    RICHMENU_ALIAS_ID_NOTFOUND,
    #[allow(non_camel_case_types)]
    RICHMENU_NOTFOUND,
    FAILED,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPlayComplete {
    pub tracking_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beacon {
    pub hwid: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub dm: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub result: String,
    pub nonce: String,
}
