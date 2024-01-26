pub use crate::{KeyboardAction, KeyboardColor};

pub trait ButtonAbstraction<T> {
    fn new(action: KeyboardAction<T>, color: Option<KeyboardColor>) -> Self;

    fn text(label: &str, payload: T, color: KeyboardColor) -> Self;

    fn open_link(link: &str, label: &str, payload: T) -> Self;

    fn location(payload: T) -> Self;

    fn vkpay(payload: T, hash: &str) -> Self;

    fn open_app(app_id: u32, owner_id: u32, payload: T, label: &str, hash: &str) -> Self;

    fn callback(label: &str, payload: T, color: KeyboardColor) -> Self;
}
