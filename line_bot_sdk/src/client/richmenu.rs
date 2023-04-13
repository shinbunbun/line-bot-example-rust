use futures::io::Empty;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{
    awc_wrapper::{SendClientRequestByteFut, SendClientRequestFut},
    models::action::Actions,
    Client,
};

use super::{API_DATA_ENDPOINT_BASE, API_ENDPOINT_BASE};

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct RichMenuObject {
    pub size: RichMenuSize,
    pub selected: bool,
    pub name: String,
    pub chat_bar_text: String,
    pub areas: Vec<RichMenuArea>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct RichMenuSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct RichMenuArea {
    pub bounds: RichMenuBounds,
    pub action: Actions,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct RichMenuBounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichMenuResponse {
    pub rich_menu_id: String,
}

impl Client {
    pub fn richmenu(&self, richmenu: RichMenuObject) -> SendClientRequestFut<RichMenuResponse> {
        SendClientRequestFut::new(self.post(
            richmenu,
            &format!("{}/v2/bot/richmenu", API_ENDPOINT_BASE),
            None,
        ))
    }

    pub fn richmenu_validate(&self, richmenu: RichMenuObject) -> SendClientRequestFut<()> {
        SendClientRequestFut::new(self.post(
            richmenu,
            &format!("{}/v2/bot/richmenu/validate", API_ENDPOINT_BASE),
            None,
        ))
    }

    pub fn richmenu_content(&self, richmenu_id: &str) -> SendClientRequestByteFut {
        SendClientRequestByteFut::new(self.post(
            (),
            &format!(
                "{}/v2/bot/richmenu/{}/content",
                API_DATA_ENDPOINT_BASE, richmenu_id
            ),
            None,
        ))
    }
}
