use serde::Deserialize;
use vec1::Vec1;

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SimpleText {
    pub simple_text: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Thumbnail {
    pub url: String,
    pub width: u16,
    pub height: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SimpleThumbnail {
    pub url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Image {
    pub thumbnails: Vec1<Thumbnail>,
}

impl Image {
    pub fn get_first(self) -> Thumbnail {
        let (first, _) = self.thumbnails.split_off_first();
        first
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CustomImage {
    pub thumbnails: Vec1<SimpleThumbnail>,
}

impl CustomImage {
    pub fn get_first(self) -> SimpleThumbnail {
        let (first, _) = self.thumbnails.split_off_first();
        first
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Message {
    pub runs: Vec1<MessageContent>,
}

#[derive(Debug)]
pub enum MessageContent {
    Text(String),
    Emoji(Emoji),
    Link {
        text: String,
        url: String
    }
}

impl<'de> Deserialize<'de> for MessageContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct EmojiStruct {
            shortcuts: Vec1<String>,
            image: Image,
            is_custom_emoji: bool,
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct MessageStruct {
            text: Option<String>,
            navigation_endpoint: Option<NavigationStruct>,
            emoji: Option<EmojiStruct>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct NavigationStruct {
            url_endpoint: Option<UrlStruct>,
            watch_endpoint: Option<WatchStruct>
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct UrlStruct {
            url: String
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct WatchStruct {
            video_id: String
        }

        let message_struct = MessageStruct::deserialize(deserializer)?;

        if message_struct.text.is_some() && message_struct.emoji.is_some() {
            return Err(serde::de::Error::custom("both `text` and `emoji` are present"));
        }

        if message_struct.emoji.is_some() && message_struct.navigation_endpoint.is_some() {
            return Err(serde::de::Error::custom("both `emoji` and `navigationEndpoint` are present"));
        }

        if message_struct.text.is_none() && message_struct.navigation_endpoint.is_some() {
            return Err(serde::de::Error::custom("have `navigationEndpoint`, but no `text`"));
        }

        if let Some(text) = message_struct.text {
            if let Some(navigation_endpoint) = message_struct.navigation_endpoint {
                if navigation_endpoint.url_endpoint.is_some() && navigation_endpoint.watch_endpoint.is_some() {
                    return Err(serde::de::Error::custom("have both `urlEndpoint` and `watchEndpoint`"));
                }

                let url = if let Some(url_endpoint) = navigation_endpoint.url_endpoint {
                    format!("https://www.youtube.com{}", url_endpoint.url)
                } else if let Some(watch_struct) = navigation_endpoint.watch_endpoint {
                    format!("https://www.youtube.com/watch?v={}", watch_struct.video_id)
                } else {
                    return Err(serde::de::Error::custom("no `urlEndpoint` nor `watchEndpoint`"));
                };

                return Ok(MessageContent::Link{ text, url});
            } else {
                return Ok(MessageContent::Text(text));
            }
        }

        if let Some(emoji) = message_struct.emoji {
            let (label, _) = emoji.shortcuts.split_off_first();

            let emoji = Emoji {
                image: emoji.image,
                is_custom_emoji: emoji.is_custom_emoji,
                label
            };

            return Ok(MessageContent::Emoji(emoji));
        }

        Err(serde::de::Error::custom("couldn't deserialize"))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Emoji {
    pub image: Image,
    pub is_custom_emoji: bool,
    pub label: String
}

#[derive(Debug)]
pub struct AuthorBadge {
    pub badge_type: BadgeType,
    pub tooltip: String,
}

impl<'de> Deserialize<'de> for AuthorBadge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        #[derive(Deserialize)]
        struct Outer {
            #[serde(rename = "liveChatAuthorBadgeRenderer")]
            inner: Inner,
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct Inner {
            #[serde(flatten)]
            badge_type: BadgeType,
            tooltip: String,
        }

        let outer = Outer::deserialize(deserializer)?;
        Ok(AuthorBadge {
            badge_type: outer.inner.badge_type,
            tooltip: outer.inner.tooltip
        })
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum BadgeType {
    Icon(Icon),
    CustomThumbnail(CustomImage),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Icon {
    pub icon_type: IconType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub enum IconType {
    Verified,
    Owner,
    Moderator,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AuthorInfo {
    pub author_photo: Image,
    pub author_name: Option<SimpleText>,
    pub author_external_channel_id: String,
    pub author_badges: Option<Vec1<AuthorBadge>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ContextMenu {
    live_chat_item_context_menu_endpoint: ContextMenuEndpoint
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ContextMenuEndpoint {
    params: String
}