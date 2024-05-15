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
