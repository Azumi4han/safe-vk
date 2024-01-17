mod longpoll;
mod update;

pub use longpoll::*;
pub use update::*;

impl Default for Ctx {
    fn default() -> Self {
        Ctx {
            ts: String::from(""),
            updates: vec![],
        }
    }
}
