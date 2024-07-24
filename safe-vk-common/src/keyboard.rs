use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum KeyboardAction<T> {
    Text {
        #[serde(rename = "type")]
        button_type: String,
        label: String,
        payload: T,
    },
    OpenLink {
        #[serde(rename = "type")]
        button_type: String,
        link: String,
        label: String,
        payload: T,
    },
    Location {
        #[serde(rename = "type")]
        button_type: String,
        payload: T,
    },
    VkPay {
        #[serde(rename = "type")]
        button_type: String,
        payload: T,
        hash: String,
    },
    OpenApp {
        #[serde(rename = "type")]
        button_type: String,
        app_id: u32,
        owner_id: u32,
        payload: T,
        label: String,
        hash: String,
    },
    Callback {
        #[serde(rename = "type")]
        button_type: String,
        label: String,
        payload: T,
    },
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum KeyboardColor {
    Primary,
    Secondary,
    Negative,
    Positive,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Keyboard<T> {
    one_time: bool,
    inline: bool,
    buttons: Vec<Button<T>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Button<T> {
    action: KeyboardAction<T>,
    color: Option<KeyboardColor>,
}

// Button colors are only for 'text' and 'callback' buttons
impl<T> Button<T> {
    pub fn new(action: KeyboardAction<T>, color: Option<KeyboardColor>) -> Self {
        Button { action, color }
    }

    pub fn text(label: &str, payload: T, color: KeyboardColor) -> Self {
        let button_type = String::from("text");
        Button::new(
            KeyboardAction::Text {
                button_type,
                label: label.to_string(),
                payload,
            },
            Some(color),
        )
    }

    pub fn open_link(link: &str, label: &str, payload: T) -> Self {
        let button_type = String::from("open_link");
        Button::new(
            KeyboardAction::OpenLink {
                button_type,
                link: link.to_string(),
                label: label.to_string(),
                payload,
            },
            None,
        )
    }

    pub fn location(payload: T) -> Self {
        let button_type = String::from("location");
        Button::new(
            KeyboardAction::Location {
                button_type,
                payload,
            },
            None,
        )
    }

    pub fn vkpay(payload: T, hash: &str) -> Self {
        let button_type = String::from("vkpay");
        Button::new(
            KeyboardAction::VkPay {
                button_type,
                payload,
                hash: hash.to_string(),
            },
            None,
        )
    }

    pub fn open_app(app_id: u32, owner_id: u32, payload: T, label: &str, hash: &str) -> Self {
        let button_type = String::from("open_app");
        Button::new(
            KeyboardAction::OpenApp {
                button_type,
                app_id,
                owner_id,
                payload,
                label: label.to_string(),
                hash: hash.to_string(),
            },
            None,
        )
    }

    pub fn callback(label: &str, payload: T, color: KeyboardColor) -> Self {
        let button_type = String::from("callback");
        Button::new(
            KeyboardAction::Callback {
                button_type,
                label: label.to_string(),
                payload,
            },
            Some(color),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowSnackbar {
    /// The type of action, always set to 'show_snackbar'.
    #[serde(rename = "type")]
    event_type: &'static str,
    /// The text to display in the snackbar. Maximum length is 90 characters.
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenLink {
    /// The type of action, always set to 'open_link'.
    #[serde(rename = "type")]
    event_type: &'static str,
    /// The URL to be opened.
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApp {
    /// The type of action, always set to 'open_app'.
    #[serde(rename = "type")]
    event_type: &'static str,
    /// The app identifier for the VK Mini Apps.
    app_id: i32,
    /// The identifier of the community where the app is installed, if it is opened in a community context.
    owner_id: i32,
    /// The navigation hash that will be added to the launch parameters after the '#' symbol.
    ///
    /// Do not include the '#' symbol in the hash.
    hash: String,
}

impl ShowSnackbar {
    /// Creates a new `ShowSnackbar` action.
    pub fn new(text: impl Into<String>) -> Self {
        ShowSnackbar {
            event_type: "show_snackbar",
            text: text.into(),
        }
    }
}

impl OpenLink {
    /// Creates a new `OpenLink` action.
    pub fn new(link: impl Into<String>) -> Self {
        OpenLink {
            event_type: "open_link",
            link: link.into(),
        }
    }
}

impl OpenApp {
    /// Creates a new `OpenApp` action with the specified parameters.
    pub fn new(app_id: i32, owner_id: i32, hash: String) -> Self {
        OpenApp {
            event_type: "open_app",
            app_id,
            owner_id,
            hash,
        }
    }
}
