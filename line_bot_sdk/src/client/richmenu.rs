use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{
    awc_wrapper::{SendClientRequestByteFut, SendClientRequestFut},
    models::action::Actions,
    Client,
};

use super::{API_DATA_ENDPOINT_BASE, API_ENDPOINT_BASE};

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuObject {
    pub size: RichMenuSize,
    pub selected: bool,
    pub name: String,
    pub chat_bar_text: String,
    pub areas: Vec<RichMenuArea>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuArea {
    pub bounds: RichMenuBounds,
    pub action: Actions,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuBounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRichMenuResponse {
    pub rich_menu_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuResponseObject {
    pub richmenu_id: String,
    pub size: RichMenuSize,
    pub selected: bool,
    pub name: String,
    pub chat_bar_text: String,
    pub areas: Vec<RichMenuArea>,
}

impl Client {
    pub fn post_richmenu(
        &self,
        richmenu: RichMenuObject,
    ) -> SendClientRequestFut<PostRichMenuResponse> {
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

    pub fn richmeu_list(&self) -> SendClientRequestFut<RichMenuResponseObject> {
        SendClientRequestFut::new(self.get(
            &format!("{}/v2/bot/richmenu/list", API_ENDPOINT_BASE),
            None::<&[(); 0]>,
            None,
            true,
        ))
    }

    pub fn get_richmenu(&self, richmenu_id: &str) -> SendClientRequestFut<RichMenuResponseObject> {
        SendClientRequestFut::new(self.get(
            &format!("{}/v2/bot/richmenu/{}", API_ENDPOINT_BASE, richmenu_id),
            None::<&[(); 0]>,
            None,
            true,
        ))
    }

    pub fn delete_richmenu(&self, richmenu_id: &str) -> SendClientRequestFut<()> {
        SendClientRequestFut::new(self.delete(
            &format!("{}/v2/bot/richmenu/{}", API_ENDPOINT_BASE, richmenu_id),
            None::<&[(); 0]>,
        ))
    }
}