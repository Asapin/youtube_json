use serde::Deserialize;
use vec1::Vec1;

use super::root::Continuation;

#[derive(Debug)]
pub struct Header {
    pub view_selector: Vec1<MenuItems>
}

impl<'de> Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        #[derive(Deserialize)]
        struct TempHeader {
            #[serde(rename = "liveChatHeaderRenderer")]
            renderer: Renderer,
        }

        #[derive(Deserialize)]
        struct Renderer {
            #[serde(rename = "viewSelector")]
            selector: Selector
        }

        #[derive(Deserialize)]
        struct Selector {
            #[serde(rename = "sortFilterSubMenuRenderer")]
            filter_renderer: FilterRenderer
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct FilterRenderer {
            sub_menu_items: Vec1<MenuItems>
        }

        let temp_header = TempHeader::deserialize(deserializer)?;
        Ok(Header {
            view_selector: temp_header.renderer.selector.filter_renderer.sub_menu_items
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MenuItems {
    pub title: String,
    pub subtitle: String,
    pub selected: bool,
    pub continuation: Continuation
}