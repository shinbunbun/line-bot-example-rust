use serde::{Deserialize, Serialize};

use crate::models::action::{Actions, URIAction};

use super::Message;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub alt_text: String,
    pub contents: FlexContainer,
}

impl Message<'_> for FlexMessage {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum FlexContainer {
    Bubble(FlexBubble),
    Carousel(FlexCarousel),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexBubble {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<FlexBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero: Option<FlexHero>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<FlexBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<FlexBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<FlexBubbleStyles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Actions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum FlexHero {
    Box(FlexBox),
    Image(FlexImage),
    Video(FlexVideo),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexBubbleStyles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<FlexBlockStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero: Option<FlexBlockStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<FlexBlockStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<FlexBlockStyle>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexBlockStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_color: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexCarousel {
    #[serde(rename = "type")]
    pub type_field: String,
    pub contents: Vec<FlexBubble>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexBox {
    #[serde(rename = "type")]
    pub type_field: String,
    pub layout: String,
    pub contents: Vec<FlexBoxComponent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_all: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_top: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_top: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Actions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justify_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align_items: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<FlexBoxBackground>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum FlexBoxComponent {
    Box(FlexBox),
    Button(FlexButton),
    Image(FlexImage),
    Text(FlexText),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexBoxBackground {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center_position: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexButton {
    #[serde(rename = "type")]
    pub type_field: String,
    pub action: Actions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_top: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexImage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_top: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Actions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexVideo {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub preview_url: String,
    pub alt_content: FlexVideoComponent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<URIAction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum FlexVideoComponent {
    Box(FlexBox),
    Image(FlexImage),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexIcon {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_top: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexText {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<FlexSpan>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_top: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Actions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoration: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexSpan {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoration: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexSeparator {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexFiller {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex: Option<i64>,
}
