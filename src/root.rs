use serde::{Deserialize, Serialize};
use vec1::Vec1;
use super::{actions::{Action, OptionalAction}, header::Header, participants::ParticipantsList};

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct InitialChatJson {
    pub contents: Option<ChatContents>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ChatContents {
    pub live_chat_renderer: LiveChat,
}

#[derive(Debug)]
pub struct LiveChat {
    pub continuations: Vec1<Continuation>,
    pub actions: Option<Vec1<Action>>,
    pub participants_list: Option<ParticipantsList>,
    pub header: Option<Header>
}

impl<'de> Deserialize<'de> for LiveChat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct Inner {
            continuations: Vec1<Continuation>,
            actions: Option<Vec1<OptionalAction>>,
            participants_list: Option<ParticipantsList>,
            header: Option<Header>
        }

        let inner: Inner = Inner::deserialize(deserializer)?;

        let actions = match inner.actions {
            Some(actions) => {
                let actions: Vec<Action> = actions
                    .into_iter()
                    .filter_map(|action| {
                        match action {
                            OptionalAction::Action(action) => Option::Some(action),
                            OptionalAction::None => Option::None
                        }
                    })
                    .collect();
                if actions.is_empty() {
                    None
                } else {
                    Some(actions)
                }
            }
            None => Option::None
        };

        let actions = match actions {
            Some(actions) => {
                let vec1_actions = vec1::Vec1::try_from_vec(actions)
                    .map_err(|_e| serde::de::Error::custom("Couldn't create vec1 from vec of actions"))?;
                Some(vec1_actions)
            },
            None => None
        };

        Ok(
            LiveChat {
                continuations: inner.continuations,
                actions,
                participants_list: inner.participants_list,
                header: inner.header
            }
        )
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ChatJson {
    pub continuation_contents: Option<ContinuationContents>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ContinuationContents {
    pub live_chat_continuation: LiveChat,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum Continuation {
    #[serde(rename_all(deserialize = "camelCase"))]
    TimedContinuationData {
        timeout_ms: u16,
        continuation: String,
    },
    #[serde(rename_all(deserialize = "camelCase"))]
    InvalidationContinuationData {
        timeout_ms: u16,
        continuation: String,
    },
    #[serde(rename_all(deserialize = "camelCase"))]
    ReloadContinuationData {
        continuation: String,
    }
}

impl Continuation {
    pub fn get_timeout_and_continuation(self) -> (u16, String) {
        match self {
            Continuation::TimedContinuationData { timeout_ms, continuation } => (timeout_ms, continuation),
            Continuation::InvalidationContinuationData { timeout_ms, continuation } => (timeout_ms, continuation),
            Continuation::ReloadContinuationData { continuation } => (0, continuation)
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YoutubeParams {
    context: ParamsContext,
    continuation: String,
    web_client_info: WebClientInfo
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamsContext {
    client: ClientParams,
    request: RequestParams,
    #[serde(default = "YoutubeParams::default_user")]
    user: UserParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_screen_nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    click_tracking: Option<ClickTracking>,
    #[serde(default = "YoutubeParams::ad_signals_info")]
    ad_signals_info: AdSignalsInfo
}

impl ParamsContext {
    pub fn update_event_id(&mut self, event_id: Option<String>) {
        self.client_screen_nonce = event_id;
    }

    pub fn update_referer(&mut self, referer: String) {
        self.client.main_app_web_info.graft_url = referer;
    }

    pub fn get_mut_ad_signals_info(&mut self) -> &mut AdSignalsInfo {
        &mut self.ad_signals_info
    }

    pub fn get_client_params(&self) -> &ClientParams {
        &self.client
    }

    pub fn set_connection_type(&mut self, connection_type: Option<String>) {
        self.client.connection_type = connection_type;
    }

    pub fn height(&self) -> u16 {
        self.client.screen_height_points
    }

    pub fn width(&self) -> u16 {
        self.client.screen_width_points
    }

    pub fn get_visitor_data<'a>(&'a self) -> &'a str {
        &self.client.visitor_data
    }

    pub fn set_click_tracking(&mut self, param: Option<String>) {
        match param {
            Some(param) => {
                self.click_tracking = Some(
                    ClickTracking {
                        click_tracking_params: param
                    }
                );
            }
            None => {
                self.click_tracking = None;
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientParams {
    hl: String,
    gl: String,
    visitor_data: String,
    user_agent: String,
    client_name: String,
    client_version: String,
    os_name: String,
    os_version: String,
    browser_name: String,
    browser_version: String,
    #[serde(default = "YoutubeParams::default_screen_width")]
    screen_width_points: u16,
    #[serde(default = "YoutubeParams::default_screen_height")]
    screen_height_points: u16,
    #[serde(default = "YoutubeParams::default_pixel_density")]
    screen_pixel_density: u16,
    #[serde(default = "YoutubeParams::screen_density_float")]
    screen_density_float: f32,
    #[serde(default = "YoutubeParams::default_utc_offset")]
    utc_offset_minutes: i16,
    #[serde(default = "YoutubeParams::default_interface_theme")]
    user_interface_theme: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<String>,
    #[serde(default = "YoutubeParams::default_app_web_info")]
    main_app_web_info: MainAppWebInfo,
    #[serde(default = "YoutubeParams::default_time_zone")]
    time_zone: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestParams {
    session_id: String,
    #[serde(default = "Vec::new")]
    internal_experiment_flags: Vec<String>,
    #[serde(default = "Vec::new")]
    consistency_token_jars: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserParams {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClickTracking {
    click_tracking_params: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdSignalsInfo {
    params: Vec<CustomParams>
}

impl AdSignalsInfo {
    pub fn add_param(&mut self, key: String, value: String) {
        let param = CustomParams {
            key,
            value
        };

        self.params.push(param);
    }

    pub fn clear_params(&mut self) {
        self.params = Vec::new();
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomParams {
    key: String,
    value: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebClientInfo {
    is_document_hidden: bool
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainAppWebInfo {
    graft_url: String
}

impl YoutubeParams {
    pub fn new_youtube_params(context: ParamsContext) -> YoutubeParams {
        let web_client_info = WebClientInfo {
            is_document_hidden: false
        };

        YoutubeParams {
            context,
            continuation: "".to_string(),
            web_client_info
        }
    }

    pub fn update_event_id(mut self, event_id: Option<String>) -> YoutubeParams {
        self.context.update_event_id(event_id);
        self
    }

    pub fn update_continuation(&mut self, continuation: String) {
        self.continuation = continuation;
    }

    pub fn update_referer(&mut self, referer: String) {
        self.context.update_referer(referer);
    }

    fn default_screen_width() -> u16 {
        401
    }

    fn default_screen_height() -> u16 {
        566
    }

    fn default_pixel_density() -> u16 {
        1
    }

    fn screen_density_float() -> f32 {
        1.0
    }

    fn default_utc_offset() -> i16 {
        0
    }

    fn default_interface_theme() -> String {
        "USER_INTERFACE_THEME_LIGHT".to_string()
    }

    fn default_user() -> UserParams {
        UserParams {}
    }

    fn default_app_web_info() -> MainAppWebInfo {
        MainAppWebInfo {
            graft_url: "".to_string()
        }
    }

    fn default_time_zone() -> String {
        "Europe/London".to_string()
    }

    fn ad_signals_info() -> AdSignalsInfo {
        AdSignalsInfo {
            params: Vec::new()
        }
    }
}
