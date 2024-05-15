use crate::Filter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListenerMethod {
    Watch,
    Command {
        update_type: &'static str,
        trigger: String,
        filter: Filter,
    },
}

impl ListenerMethod {
    pub fn command(trigger: String, filter: Filter) -> Self {
        ListenerMethod::Command {
            update_type: "message_new",
            trigger,
            filter,
        }
    }
}
