mod event_answer;
mod longpoll;
mod update;
mod user;

pub use event_answer::*;
pub use longpoll::*;
pub use update::*;
pub use user::*;

impl Default for Ctx {
    fn default() -> Self {
        Ctx {
            ts: String::from(""),
            updates: vec![],
        }
    }
}
