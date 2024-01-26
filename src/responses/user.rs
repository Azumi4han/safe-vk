use serde::Deserialize;
use serde_json::Value;

/// Information about the user’s career
#[derive(Deserialize, Debug)]
pub struct Career {
    /// Community ID (if available, otherwise company);
    pub group_id: Option<u64>,
    /// Company name (if available, otherwise group_id);
    pub company: Option<String>,
    /// Country identifier
    pub country_id: Option<u64>,
    /// Name of the city (if available, otherwise) city_id;
    pub city_name: Option<String>,
    /// City ID (if available, otherwise city_name);
    pub city_id: Option<u64>,
    /// Year of commencement of work
    pub from: Option<u64>,
    /// The year of termination of employment
    pub until: Option<u64>,
    /// Position
    pub position: Option<String>,
}

/// Information about the city indicated on the user's page in the section "Contacts"
#[derive(Deserialize, Debug)]
pub struct City {
    /// City ID, which can be used to get its name using the method `database.getCitiesById`
    pub id: Option<u64>,
    /// The name of the city
    pub title: Option<String>,
}

/// Information about the user's phone numbers
#[derive(Deserialize, Debug)]
pub struct Contacts {
    /// Mobile phone number of the user (only for Standalone applications)
    pub mobile_phone: Option<String>,
    /// Additional phone number of the user
    pub home_phone: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Counters {
    /// Number of photo albums
    pub albums: Option<u64>,
    /// Number of videos
    pub videos: Option<u64>,
    /// The number of audio recordings
    pub audios: Option<u64>,
    /// Number of photographs
    pub photos: Option<u64>,
    /// Number of notes
    pub notes: Option<u64>,
    /// Number of friends
    pub friends: Option<u64>,
    /// The number of user gifts
    pub gifts: Option<u64>,
    /// The number of communities
    pub groups: Option<u64>,
    /// Number of online friends
    pub online_friends: Option<u64>,
    /// Number of common friends
    pub mutual_friends: Option<u64>,
    /// The number of videos with the user
    pub user_videos: Option<u64>,
    /// Number of photos with the user
    pub user_photos: Option<u64>,
    /// Number of subscribers
    pub followers: Option<u64>,
    /// The number of objects in the block "Interesting pages"
    pub pages: Option<u64>,
    /// Number of subscriptions of the user (to whom the user is subscribed)
    pub subscriptions: Option<u64>,
}

/// Information about the country indicated on the user's page in the section "Contacts"
#[derive(Deserialize, Debug)]
pub struct Country {
    /// Country identifier that can be used to get its name using the method database.getCountriesById
    pub id: Option<u64>,
    /// The name of the country.
    pub title: Option<String>,
}

/// Information about the higher educational institution of the user.
#[derive(Deserialize, Debug)]
pub struct Education {
    /// University ID
    pub university: Option<u64>,
    /// The name of the university
    pub university_name: Option<String>,
    /// ID of the faculty
    pub faculty: Option<u64>,
    /// The name of the faculty
    pub faculty_name: Option<String>,
    /// Year of end
    pub graduation: Option<u64>,
}

/// Time of last visit. An object containing the following fields
#[derive(Deserialize, Debug)]
pub struct LastSeen {
    /// Time of last visit in Unixtime format
    pub time: Option<u64>,
    /// Type of platform. Possible values:
    /// - `1` if the mobile version
    /// - `2` if application for iPhone
    /// - `3` if app for iPad
    /// - `4` if application for Android
    /// - `5` if application for Windows Phone
    /// - `6` if application for Windows 10
    /// - `7` if full version of the site
    pub platform: Option<u64>,
}

/// Information about the military service of the user
#[derive(Deserialize, Debug)]
pub struct Military {
    /// The part number
    pub unit: Option<String>,
    /// The part identifier in the database
    pub unit_id: Option<u64>,
    /// The country in which the part is located
    pub country_id: Option<u64>,
    /// year of commencement of service
    pub from: Option<u64>,
    /// Year of end of service
    pub until: Option<u64>,
}

/// Information about the current type of user activity
#[derive(Deserialize, Debug)]
pub struct Occupation {
    /// Is a type. Possible values
    /// - `work` if working
    /// - `school` if secondary education
    /// - `university` if higher education
    #[serde(rename = "type")]
    pub occupation_type: Option<String>,
    /// ID of the school, university, community of the company (in which the user works)
    pub id: Option<u64>,
    /// The name of the school, university or place of work
    pub name: Option<String>,
}

///
/// The object contains information about the VKontakte user. The set of fields can change
/// depending on the method called and the parameters transmitted in it.
///
/// Note that all fields use information about the current user (e.g., blacklisted_by_me require a
/// user access key to be passed in the request, even if the method itself can be invoked without
/// an access key.
///
#[derive(Deserialize, Debug)]
pub struct User {
    /// User ID
    pub id: u64,
    /// First name
    pub first_name: String,
    /// Last name
    pub last_name: String,
    /// The field is returned if the user’s page is deleted or blocked,
    /// contains a value `deleted` or `banned`
    pub deactivated: Option<String>,
    /// Whether the user profile is hidden by privacy settings
    pub is_closed: bool,
    /// Can the current user see the profile `is_closed` = 1 (They are friends)
    pub can_access_closed: bool,
    ///
    /// Here comes `optional` fields that are not always included in respons
    ///
    /// The content of the field "ABOUT yourself" from the profile
    pub about: Option<String>,
    /// Content of the Activity field from the profile
    pub activities: Option<String>,
    /// Date of birth. Returns in D.M.YYYY or D.M format (if birth year is hidden).
    /// If the date of birth is hidden in its entirety,
    /// the field is missing from the answer
    pub dbate: Option<String>,
    /// Information about whether the current user is blacklisted.
    /// Possible values:
    /// - `1` if the user is blacklisted.
    /// - `0` if the user is not blacklisted.
    pub blacklisted: Option<u8>,
    /// Information about whether the user is blacklisted from the current user.
    /// Possible values:
    /// - `1` if it is
    /// - `0` if it isn't
    pub blacklisted_by_me: Option<u8>,
    /// The content of the “Favorite Books” field from the user profile
    pub books: Option<String>,
    /// Information about whether the current user can leave records on the wall.
    /// Possible values:
    /// - `1` if maybe he can
    /// - `0` if he can't
    pub can_post: Option<u8>,
    /// Information about whether the current user can see someone else's records on the wall.
    /// Possible values:
    /// - `1` if maybe he can
    /// - `0` if he can't
    pub can_see_all_posts: Option<u8>,
    /// Information about whether the current user can see the audio recordings.
    /// Possible values:
    /// - `1` if maybe he can
    /// - `0` if he can't
    pub can_see_audio: Option<u8>,
    /// Information about whether a notification will be sent to the user about the friend request from the current user.
    /// Possible values:
    /// - `1` notification will be sent
    /// - `0` notification will not be sent
    pub can_send_friend_request: Option<u8>,
    /// Information about whether the current user can send a private message.
    /// Possible values:
    /// - `1` if maybe he can
    /// - `0` if maybe he can't
    pub can_write_private_message: Option<u8>,
    /// Information about the user’s career. An object containing the following fields
    pub career: Option<Career>,
    /// Information about the city indicated on the user's page in the section "Contacts".
    pub city: Option<City>,
    /// Shared friends with current user
    pub common_count: Option<u64>,
    /// Returns data about the services specified in the user profile, such as: skype, livejournal.
    /// For each service, a separate field with a string type containing the username of the user is returned.
    /// For example, "skype": "username"
    pub connections: Option<Value>,
    /// Information about the user's phone numbers. If the data is specified and not hidden by
    /// the privacy settings
    pub contacts: Option<Contacts>,
    /// The number of different objects of the user. The field only returns in method users.get when
    /// requesting information about one user, with the transfer of user information access_token.
    pub counters: Option<Counters>,
    /// Information about the country indicated on the user's page in the section "Contacts".
    pub country: Option<Country>,
    /// Returns data on the points at which profile and miniature photos of the user are cut, if available
    pub crop_photo: Option<Value>,
    /// Short address of the page. A line containing a short page address is returned (e.g., andrew ).
    /// If he is not appointed, he shall return "id" + user_id , for example: id35828305
    pub domain: Option<String>,
    /// Information about the higher educational institution of the user
    pub education: Option<Education>,
    /// External services to which exports from VK are configured `livejournal`.
    pub exports: Option<Value>,
    /// Name in nominative case (e.g., "Alexander").
    pub first_name_nom: Option<String>,
    /// Name in genitive case (e.g., "Alexandera").
    pub first_name_gen: Option<String>,
    /// Name in dative case (e.g., "Alexandru").
    pub first_name_dat: Option<String>,
    /// Name in accusative case (e.g., "Alexandra").
    pub first_name_acc: Option<String>,
    /// Name in instrumental case (e.g., "Alexandrom").
    pub first_name_ins: Option<String>,
    /// Name in prepositional case (e.g., "Alexandre").
    pub first_name_abl: Option<String>,
    /// Number of subscribers of the user
    pub followers_count: Option<u64>,
    /// Friendship status with the user.
    /// Possible values:
    /// - `0` if it's not a friend
    /// - `1` if sent an application / subscription to the user
    /// - `2` if there is an incoming application / subscription from the user
    /// - `3` if is a friend
    pub friend_status: Option<u64>,
    /// Content of the “Favorite Games” field from the profile
    pub games: Option<String>,
    /// Information about whether the user's mobile phone number is known.
    /// Return values:
    /// - `1` if known
    /// - `0` if unknown
    pub has_mobile: Option<bool>,
    /// Information about whether the user has set a profile photo.
    /// Return values:
    /// - `1` if established
    /// - `0` if didn't
    pub has_photo: Option<u64>,
    /// Name of hometown
    pub home_town: Option<String>,
    /// Content of the “Interesa” field from the profile
    pub interests: Option<String>,
    /// Information about whether the user is bookmarked by the current user.
    /// Possible values:
    /// - `1` if it's
    /// - `0` if it's not
    pub is_favorite: Option<u64>,
    /// Information about whether the user is a friend of the current user.
    /// Possible values:
    /// - `1` if true
    /// - `0` if false
    pub is_friend: Option<u64>,
    /// Information about whether the user is hidden from the news feed of the current user.
    /// Possible values:
    /// - `1` if true
    /// - `0` if false
    pub is_hidden_from_feed: Option<u64>,
    /// Is the profile indexed by search sites??
    /// Possible values:
    /// - `1` if profile is hidden from search sites
    /// - `0` if profile is available to search sites. (IN privacy settings: https://vk.com/settings?act=privacy,
    /// in the item “Who can see my page on the Internet", the value “Everyone” is selected
    ///
    ///
    /// Optional fields L-R
    ///
    ///
    /// Name in nominative case (e.g., "Alexander")
    pub last_name_nom: Option<String>,
    /// Name in genitive case (e.g., "Alexandera")
    pub last_name_gen: Option<String>,
    /// Name in dative case (e.g., "Alexandru")
    pub last_name_dat: Option<String>,
    /// Name in accusative case (e.g., "Alexandra")
    pub last_name_acc: Option<String>,
    /// Name in instrumental case (e.g., "Alexandrom")
    pub last_name_ins: Option<String>,
    /// Name in prepositional case (e.g., "Alexandre")
    pub last_name_abl: Option<String>,
    /// Time of last visit
    pub last_seen: Option<LastSeen>,
    /// The comma-separated identifiers of the user's friend lists. The field is only available for the method friends.get
    pub lists: Option<String>,
    /// My maiden name
    pub maiden_name: Option<String>,
    /// Information about the military service of the user
    pub military: Option<Military>,
    /// The content of the “Favorite Movies” field from the user profile
    pub movies: Option<String>,
    /// The content of the field “Favorite music” from the user profile
    pub music: Option<String>,
    /// Nickname of the user
    pub nickname: Option<String>,
    /// Information about the current type of user activity
    pub occupation: Option<Occupation>,
    /// Information about whether the user is currently on the site.
    /// If the user uses a mobile application or mobile version, an additional field is returned
    /// `online_mobile` which contains `1` . In this case, if the application is used,
    /// the field is additionally returned. `online_app` containing its identifier
    pub online: Option<u64>,
    ///
    ///
    /// Option fields S-W
    ///
    ///
    /// The short name of the page
    pub screen_name: Option<String>,
    /// Possible values:
    /// - `1` if female
    /// - `2` if male
    /// - `0` unknown
    pub sex: Option<u64>,
    /// The address of the site indicated in the profile
    pub site: Option<String>,
    /// Status of user. The line containing the status text located in the profile under the name is returned.
    /// If the option “Translate to status playing music” is enabled, an additional field is returned
    /// `status_audio` containing information about the composition
    pub status: Option<String>,
    /// Information about whether the user's page has a "light"
    pub trending: Option<u64>,
    /// Favorite TV show.
    pub tv: Option<String>,
    /// It's coming back
    /// - `1` if the user page is verified
    /// - `0` if not
    pub verified: Option<u64>,
    /// The default wall mode
    /// Possible values:
    /// - `owner`
    /// - `all`
    pub wall_default: Option<String>,
}
