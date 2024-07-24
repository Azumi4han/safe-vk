use super::Shape;
use serde_json::Error as SerdeJsonError;

#[rustfmt::skip]
#[derive(thiserror::Error, Debug)]
pub enum VkError {
    #[error("Unknown error occurred. Try to repeat the request later.\nAdditional details: {0}")]
    UnknownError(String),

    #[error("The application is disabled. VK Error: {0}")]
    ApplicationDisabled(String),

    #[error("An unknown method was passed. Check that the method name is correct: vk.com/dev/methods.\nAdditional details: {0}")]
    UnknownMethod(String),

    #[error("Invalid signature. VK Error: {0}")]
    InvalidSignature(String),

    #[error("User authorization failed. Ensure that you are using the correct authorization scheme.\nAdditional details: {0}")]
    UserAuthorizationFailed(String),

    #[error("Too many requests per second. Increase the request interval or use the execute method. More info: vk.com/dev/api_requests.\nAdditional details: {0}")]
    TooManyRequests(String),

    #[error("No permission to perform this action. Check that the necessary permissions have been obtained during authorization.\nAdditional details: {0}")]
    NoPermissionForAction(String),

    #[error("Invalid request. Check the syntax of the request and the list of used parameters.\nAdditional details: {0}")]
    InvalidRequest(String),

    #[error("Too many similar actions. Reduce the number of similar requests.\nAdditional details: {0}")]
    TooManySimilarActions(String),

    #[error("Internal server error occurred. Try to repeat the request later.\nAdditional details: {0}")]
    InternalServerError(String),

    #[error("In test mode, the application must be disabled or the user must be logged in.\nAdditional details: {0}")]
    TestModeRestrictions(String),

    #[error("Captcha needed. Process this challenge response as described here: (link to captcha handling).\nAdditional details: {0}")]
    CaptchaRequired(String),

    #[error("Access denied. Make sure you are using the correct identifiers and that the content is available to the current user on the full version of the site.\nAdditional details: {0}")]
    AccessDenied(String),

    #[error("HTTPS requests required because the user has enabled secure connection settings. Check the user's settings with account.getInfo.\nAdditional details: {0}")]
    HttpsRequired(String),

    #[error("User validation required. Redirect the user to the validation page.\nAdditional details: {0}")]
    ValidationRequired(String),

    #[error("The page has been deleted or blocked.\nAdditional details: {0}")]
    PageBlockedOrDeleted(String),

    #[error("This action is forbidden for non-Standalone applications. If your application is of type Standalone, make sure you are using redirect_uri=https://oauth.vk.com/blank.html during authorization.\nAdditional details: {0}")]
    ActionForbiddenForNonStandaloneApps(String),

    #[error("This action is only allowed for Standalone and Open API applications.\nAdditional details: {0}")]
    ActionAllowedOnlyForStandaloneAndOpenAPI(String),

    #[error("The method has been disabled. Check the current methods available here: vk.com/dev/methods.\nAdditional details: {0}")]
    MethodDisabled(String),

    #[error("User confirmation required.\nAdditional details: {0}")]
    UserConfirmationRequired(String),

    #[error("Community access token is invalid.\nAdditional details: {0}")]
    InvalidCommunityAccessToken(String),

    #[error("Application access token is invalid.\nAdditional details: {0}")]
    InvalidApplicationAccessToken(String),

    #[error("Limit on the number of method calls has been reached. See details here: https://dev.vk.com/en/api/api-requests#Restrictions%20and%20recommendations.\nAdditional details: {0}")]
    MethodCallLimitReached(String),

    #[error("The profile is private. The requested information is not available with the used access key.\nAdditional details: {0}")]
    ProfileIsPrivate(String),

    #[error("One of the required parameters was missing or incorrect. Check the required parameters and their format on the method description page.\nAdditional details: {0}")]
    MissingOrInvalidParameter(String),

    #[error("Invalid API ID of the application. Find your application in the managed list here: http://vk.com/apps?act=settings and specify the correct API ID in the request.\nAdditional details: {0}")]
    InvalidApiId(String),

    #[error("Invalid user identifier. Ensure you are using the correct identifier. Convert a short name to an ID using utils.resolveScreenName.\nAdditional details: {0}")]
    InvalidUserId(String),

    #[error("Invalid timestamp. Get the current time using `utils.getServerTime`.\nAdditional details: {0}")]
    InvalidTimestamp(String),

    #[error("Access to the album is denied. Make sure you are using the correct identifiers and that access to the requested content is available to the current user on the full version of the site.\nAdditional details: {0}")]
    AlbumAccessDenied(String),

    #[error("Access to audio is denied. Make sure you are using the correct identifiers and that access to the requested content is available to the current user on the full version of the site.\nAdditional details: {0}")]
    AudioAccessDenied(String),

    #[error("Access to the group is forbidden. Ensure the current user is a member or manager of the community (for closed and private groups and meetings).\nAdditional details: {0}")]
    GroupAccessDenied(String),

    #[error("The album is full. Remove unnecessary items from the album or use another album.\nAdditional details: {0}")]
    AlbumFull(String),

    #[error("Action is forbidden. You must enable voice translations in the application settings.\nAdditional details: {0}")]
    ActionForbidden(String),

    #[error("No rights to perform operations with the advertising cabinet.\nAdditional details: {0}")]
    NoRightsForAdOperations(String),

    #[error("An error occurred while working with the advertising cabinet.\nAdditional details: {0}")]
    AdCabinetError(String),
}

impl VkError {
    pub fn from_vk_error_json(json: &serde_json::Value) -> Self {
        let message = json
            .get("error_msg")
            .and_then(|m| m.as_str())
            .unwrap_or("No additional error message provided.")
            .to_string();
        if let Some(code) = json.get("error_code").and_then(|code| code.as_i64()) {
            match code {
                1 => Self::UnknownError(message),
                2 => Self::ApplicationDisabled(message),
                3 => Self::UnknownMethod(message),
                4 => Self::InvalidSignature(message),
                5 => Self::UserAuthorizationFailed(message),
                6 => Self::TooManyRequests(message),
                7 => Self::NoPermissionForAction(message),
                8 => Self::InvalidRequest(message),
                9 => Self::TooManySimilarActions(message),
                10 => Self::InternalServerError(message),
                11 => Self::TestModeRestrictions(message),
                14 => Self::CaptchaRequired(message),
                15 => Self::AccessDenied(message),
                16 => Self::HttpsRequired(message),
                17 => Self::ValidationRequired(message),
                18 => Self::PageBlockedOrDeleted(message),
                20 => Self::ActionForbiddenForNonStandaloneApps(message),
                21 => Self::ActionAllowedOnlyForStandaloneAndOpenAPI(message),
                23 => Self::MethodDisabled(message),
                24 => Self::UserConfirmationRequired(message),
                27 => Self::InvalidCommunityAccessToken(message),
                28 => Self::InvalidApplicationAccessToken(message),
                29 => Self::MethodCallLimitReached(message),
                30 => Self::ProfileIsPrivate(message),
                100 => Self::MissingOrInvalidParameter(message),
                101 => Self::InvalidApiId(message),
                113 => Self::InvalidUserId(message),
                150 => Self::InvalidTimestamp(message),
                200 => Self::AlbumAccessDenied(message),
                201 => Self::AudioAccessDenied(message),
                203 => Self::GroupAccessDenied(message),
                300 => Self::AlbumFull(message),
                500 => Self::ActionForbidden(message),
                600 => Self::NoRightsForAdOperations(message),
                603 => Self::AdCabinetError(message),
                _ => Self::UnknownError(message),
            }
        } else {
            Self::UnknownError(message)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("VK API error: {0}")]
    VkApi(#[from] VkError),

    /// Error to be used when `peer_id` is not found in callback from VK
    #[error("peer_id not found in VK response")]
    PeerIdNotFound,

    #[error("Unexpected JSON response: {0}")]
    UnexpectedResponse(String),

    #[error("Expected status success (1) but got uknown status {status}")]
    EventAnswerUnkownStatus { status: i8 },

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] SerdeJsonError),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Dimension index {dim} exceeds the maximum allowed shape dimensions (5x10) for shape {shape:?}")]
    DimOutOfRange { shape: Shape, dim: usize },

    /// Indicates that the listener for a specific command was not found.
    #[error("Listener not found")]
    ListenerNotFound,

    /// The event history has become outdated or has been partially lost. This can occur if the client fails to
    /// poll the server in a timely manner, causing a gap in the event sequence. The client should continue polling
    /// with the new `ts` value provided. This is not a critical error but indicates that some events might have been missed.
    #[error("LongPoll history is outdated or partially lost. To continue receiving updates, use the new 'ts' key provided: {new_ts}.")]
    EventsOutdated { new_ts: String },

    /// The session key used for the Long Poll connection has expired. This requires obtaining a new session key by
    /// calling the `groups.getLongPollServer` method again. Expired keys are common and expected to happen from time
    /// to time; they simply indicate that the connection to the Long Poll server needs to be refreshed.
    #[error("The LongPoll session key has expired. Obtain a new key by calling 'groups.getLongPollServer' again.")]
    KeyExpired,

    /// All session information has been lost, necessitating the initiation of a new Long Poll session. This error
    /// typically indicates that the server and key information are no longer valid, possibly due to a significant
    /// lapse in polling or server-side changes. Start a new session by re-fetching the server, key, and 'ts' values.
    #[error("The LongPoll session's information is lost. Start a new session by re-fetching the server, key, and 'ts' values.")]
    InformationLost,
}

pub type Result<T> = std::result::Result<T, Error>;
