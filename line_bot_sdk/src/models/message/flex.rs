use serde::{Deserialize, Serialize};

use crate::models::action::{Actions, URIAction};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub alt_text: String,
    pub contents: FlexContainer,
}

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
    pub size: Option<String>,
    pub direction: Option<String>,
    pub header: Option<FlexBox>,
    pub hero: Option<FlexHero>,
    pub body: Option<FlexBox>,
    pub footer: Option<FlexBox>,
    pub styles: Option<FlexBubbleStyles>,
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
    pub header: Option<FlexBlockStyle>,
    pub hero: Option<FlexBlockStyle>,
    pub body: Option<FlexBlockStyle>,
    pub footer: Option<FlexBlockStyle>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexBlockStyle {
    pub background_color: Option<String>,
    pub separator: Option<bool>,
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
    pub background_color: Option<String>,
    pub border_color: Option<String>,
    pub corner_radius: Option<String>,
    pub width: Option<String>,
    pub max_width: Option<String>,
    pub height: Option<String>,
    pub max_height: Option<String>,
    pub flex: Option<i64>,
    pub spacing: Option<String>,
    pub margin: Option<String>,
    pub padding_all: Option<String>,
    pub padding_top: Option<String>,
    pub padding_bottom: Option<String>,
    pub padding_start: Option<String>,
    pub padding_end: Option<String>,
    pub position: Option<String>,
    pub offset_top: Option<String>,
    pub offset_bottom: Option<String>,
    pub offset_start: Option<String>,
    pub offset_end: Option<String>,
    pub action: Option<Actions>,
    pub justify_content: Option<String>,
    pub align_items: Option<String>,
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
    pub angle: Option<String>,
    pub start_color: Option<String>,
    pub end_color: Option<String>,
    pub center_color: Option<String>,
    pub center_position: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexButton {
    #[serde(rename = "type")]
    pub type_field: String,
    pub action: Actions,
    pub flex: Option<i64>,
    pub margin: Option<String>,
    pub position: Option<String>,
    pub offset_top: Option<String>,
    pub offset_bottom: Option<String>,
    pub offset_start: Option<String>,
    pub offset_end: Option<String>,
    pub height: Option<String>,
    pub style: Option<String>,
    pub color: Option<String>,
    pub gravity: Option<String>,
    pub adjust_mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexImage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub flex: Option<i64>,
    pub margin: Option<String>,
    pub position: Option<String>,
    pub offset_top: Option<String>,
    pub offset_bottom: Option<String>,
    pub offset_start: Option<String>,
    pub offset_end: Option<String>,
    pub align: Option<String>,
    pub gravity: Option<String>,
    pub size: Option<String>,
    pub aspect_ratio: Option<String>,
    pub aspect_mode: Option<String>,
    pub background_color: Option<String>,
    pub action: Option<Actions>,
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
    pub aspect_ratio: Option<String>,
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
    pub margin: Option<String>,
    pub position: Option<String>,
    pub offset_top: Option<String>,
    pub offset_bottom: Option<String>,
    pub offset_start: Option<String>,
    pub offset_end: Option<String>,
    pub align: Option<String>,
    pub aspect_ratio: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexText {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<String>,
    pub contents: Option<Vec<FlexSpan>>,
    pub adjust_mode: Option<String>,
    pub flex: Option<i64>,
    pub margin: Option<String>,
    pub position: Option<String>,
    pub offset_top: Option<String>,
    pub offset_bottom: Option<String>,
    pub offset_start: Option<String>,
    pub offset_end: Option<String>,
    pub size: Option<String>,
    pub align: Option<String>,
    pub gravity: Option<String>,
    pub wrap: Option<bool>,
    pub line_spacing: Option<String>,
    pub max_lines: Option<i64>,
    pub weight: Option<String>,
    pub color: Option<String>,
    pub action: Option<Actions>,
    pub style: Option<String>,
    pub decoration: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexSpan {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<String>,
    pub color: Option<String>,
    pub size: Option<String>,
    pub weight: Option<String>,
    pub style: Option<String>,
    pub decoration: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexSeparator {
    #[serde(rename = "type")]
    pub type_field: String,
    pub margin: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexFiller {
    #[serde(rename = "type")]
    pub type_field: String,
    pub flex: Option<i64>,
}
