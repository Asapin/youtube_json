use serde::Deserialize;
use std::{fmt::Display, str::FromStr};
use super::generic_types::{SimpleText, Message, AuthorInfo, Image};

#[derive(Debug)]
pub enum OptionalAction {
    Action(Action),
    None
}

#[derive(Debug)]
pub enum Action {
    AddBannerToLiveChatCommand {
        banner: BannerItem
    },
    AddChatItemAction { 
        item: MessageItem 
    },
    MarkChatItemAsDeletedAction {
        deleted_state_message: Message,
        target_item_id: String,
    },
    MarkChatItemsByAuthorAsDeletedAction {
        deleted_state_message: Message,
        external_channel_id: String,
    },
    ReplaceChatItemAction {
        target_item_id: String,
        replacement_item: MessageItem,
    },
}

impl<'de> Deserialize<'de> for OptionalAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct InnerAction {
            add_banner_to_live_chat_command: Option<InnerBannerItem>,
            add_live_chat_ticker_item_action: Option<InnerChatTickerItem>,
            add_chat_item_action: Option<InnerChatItem>,
            mark_chat_item_as_deleted_action: Option<InnerDeleteItem>,
            mark_chat_items_by_author_as_deleted_action: Option<InnerBlockUserItem>,
            replace_chat_item_action: Option<InnerReplaceChatItem>,
            show_live_chat_tooltip_command: Option<InnerTooltipCommand>
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct InnerBannerItem {
            banner_renderer: BannerItem
        }

        #[derive(Deserialize)]
        struct InnerChatTickerItem { };

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct InnerChatItem {
            item: MessageItem
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct InnerDeleteItem {
            deleted_state_message: Message,
            target_item_id: String,
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct InnerBlockUserItem {
            deleted_state_message: Message,
            external_channel_id: String,
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct InnerReplaceChatItem {
            target_item_id: String,
            replacement_item: MessageItem,
        }

        #[derive(Deserialize)]
        struct InnerTooltipCommand {  }

        let inner = InnerAction::deserialize(deserializer)?;
        let mut number_of_existing_fields = 0;

        if inner.add_banner_to_live_chat_command.is_some() {
            number_of_existing_fields += 1;
        }
        if inner.add_live_chat_ticker_item_action.is_some() {
            number_of_existing_fields += 1;
        }
        if inner.add_chat_item_action.is_some() {
            number_of_existing_fields += 1;
        }
        if inner.mark_chat_item_as_deleted_action.is_some() {
            number_of_existing_fields += 1;
        }
        if inner.mark_chat_items_by_author_as_deleted_action.is_some() {
            number_of_existing_fields += 1;
        }
        if inner.replace_chat_item_action.is_some() {
            number_of_existing_fields += 1;
        }
        if inner.show_live_chat_tooltip_command.is_some() {
            number_of_existing_fields += 1;
        }

        if number_of_existing_fields == 0 {
            Err(serde::de::Error::custom(
            "Only the following actions are supported: [\
                addBannerToLiveChatCommand, \
                addLiveChatTickerItemAction, \
                addChatItemAction, \
                markChatItemAsDeletedAction, \
                markChatItemsByAuthorAsDeletedAction, \
                replaceChatItemAction, \
                showLiveChatTooltipCommand\
                ]"
            ))
        } else if number_of_existing_fields > 1 {
            Err(serde::de::Error::custom(
                "It's not possible for two actions exist simultaneously"
            ))
        } else {
            if let Some(banner) = inner.add_banner_to_live_chat_command {
                Ok(OptionalAction::Action(
                    Action::AddBannerToLiveChatCommand { banner: banner.banner_renderer }
                ))
            } else if let Some(chat_item) = inner.add_chat_item_action {
                Ok(OptionalAction::Action(
                    Action::AddChatItemAction { item: chat_item.item }
                ))
            } else if let Some(delete_item) = inner.mark_chat_item_as_deleted_action {
                Ok(OptionalAction::Action(
                    Action::MarkChatItemAsDeletedAction { 
                        deleted_state_message: delete_item.deleted_state_message,
                        target_item_id: delete_item.target_item_id
                    }
                ))
            } else if let Some(banned_user) = inner.mark_chat_items_by_author_as_deleted_action {
                Ok(OptionalAction::Action(
                    Action::MarkChatItemsByAuthorAsDeletedAction {
                        deleted_state_message: banned_user.deleted_state_message,
                        external_channel_id: banned_user.external_channel_id
                    }
                ))
            } else if let Some(replace_chat_item) = inner.replace_chat_item_action {
                Ok(OptionalAction::Action(
                    Action::ReplaceChatItemAction {
                        replacement_item: replace_chat_item.replacement_item,
                        target_item_id: replace_chat_item.target_item_id
                    }
                ))
            } else {
                Ok(OptionalAction::None)
            }
        }
    }
}

#[derive(Debug)]
pub struct BannerItem {
    pub id: String,
    pub timestamp_usec: u64,
    pub message: Message,
    pub author_info: AuthorInfo,
}

impl<'de> Deserialize<'de> for BannerItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        #[derive(Deserialize)]
        struct Outer {
            #[serde(rename = "liveChatBannerRenderer")]
            renderer: Renderer,
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct Renderer {
            contents: Content,
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct Content {
            #[serde(rename = "liveChatTextMessageRenderer")]
            entity: Entity,
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct Entity {
            id: String,
            #[serde(deserialize_with = "from_str")]
            timestamp_usec: u64,
            #[serde(flatten)]
            author_info: AuthorInfo,
            message: Message,
        }

        let outer = Outer::deserialize(deserializer)?;
        Ok(BannerItem {
            id: outer.renderer.contents.entity.id,
            timestamp_usec: outer.renderer.contents.entity.timestamp_usec,
            author_info: outer.renderer.contents.entity.author_info,
            message: outer.renderer.contents.entity.message,
        })
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum MessageItem {
    #[serde(rename_all(deserialize = "camelCase"))]
    LiveChatTextMessageRenderer {
        id: String,
        #[serde(deserialize_with = "from_str")]
        timestamp_usec: u64,
        message: Message,
        #[serde(flatten)]
        author_info: AuthorInfo,
    },
    #[serde(rename_all(deserialize = "camelCase"))]
    LiveChatMembershipItemRenderer {
        id: String,
        #[serde(deserialize_with = "from_str")]
        timestamp_usec: u64,
        #[serde(flatten)]
        author_info: AuthorInfo,
        header_subtext: Message,
    },
    #[serde(rename_all(deserialize = "camelCase"))]
    LiveChatPaidMessageRenderer {
        id: String,
        #[serde(deserialize_with = "from_str")]
        timestamp_usec: u64,
        message: Option<Message>,
        #[serde(flatten)]
        author_info: AuthorInfo,
        purchase_amount_text: SimpleText,
        header_background_color: u32,
        header_text_color: u32,
        body_background_color: u32,
        body_text_color: u32,
        author_name_text_color: u32,
        timestamp_color: u32,
    },
    #[serde(rename_all(deserialize = "camelCase"))]
    LiveChatPaidStickerRenderer {
        id: String,
        #[serde(deserialize_with = "from_str")]
        timestamp_usec: u64,
        #[serde(flatten)]
        author_info: AuthorInfo,
        sticker: Image,
        money_chip_background_color: u32,
        money_chip_text_color: u32,
        purchase_amount_text: SimpleText,
        sticker_display_width: u16,
        sticker_display_height: u16,
        background_color: u32,
        author_name_text_color: u32,
    },
    #[serde(rename_all(deserialize = "camelCase"))]
    LiveChatViewerEngagementMessageRenderer {
        id: String,
        #[serde(deserialize_with = "from_str")]
        timestamp_usec: u64,
        message: Message,
    },
    #[serde(rename_all(deserialize = "camelCase"))]
    LiveChatPlaceholderItemRenderer { 
        id: String, 
        #[serde(deserialize_with = "from_str")]
        timestamp_usec: u64 
    },
    #[serde(rename_all(deserialize = "camelCase"))]
    LiveChatModeChangeMessageRenderer {
        id: String,
        #[serde(deserialize_with = "from_str")]
        timestamp_usec: u64,
        text: Message,
        subtext: Message,
        // icon: ChatModeIcon
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ChatModeIcon {
    icon_type: ChatModeIconType
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum ChatModeIconType {
    SlowMode,
    MembersOnlyMode
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: serde::de::Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}