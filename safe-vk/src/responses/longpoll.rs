use serde::Deserialize;

/// Represents the session data needed to connect to the Long Poll server
#[derive(Deserialize, Debug)]
pub struct LongPollSession {
    /// A secret key for the session.
    pub key: String,
    /// The server URL to connect to for receiving updates.
    pub server: String,
    /// The ID of the last event received. Used to fetch subsequent events.
    pub ts: String,
}

/// Represents a response from the VK Long Poll server.
#[derive(Debug, Deserialize, Clone)]
pub struct LongPollResponse<T> {
    /// The ID of the last event received. Used in the next request to fetch subsequent events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    /// A list of events that have occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<Event<T>>>,
    /// An error code indicating that the session needs to be refreshed or restarted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
}

/// Represents an individual event in the Long Poll response.
#[derive(Debug, Deserialize, Clone)]
pub struct Event<T> {
    /// The type of event (e.g., message_new, group_join).
    #[serde(rename = "type")]
    pub update_type: String,
    /// A unique identifier for the event.
    pub event_id: String,
    /// The API version for which the event is formatted.
    pub v: String,
    /// The object that initiated the event, varying in structure depending on the event type.
    pub object: T,
    // /// The ID of the community where the event occurred.
    // pub object_id: i64,
}
