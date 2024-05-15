use super::message::Geo;
use crate::Button;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Attachment<T> {
    /// Type of the attachment. Possible values are:
    /// - "photo" for photographs
    /// - "video" for video recordings
    /// - "audio" for audio recordings
    /// - "doc" for documents
    /// - "link" for external links
    /// - "market" for marketplace items (available from API version 5.52)
    /// - "market_album" for collections of marketplace items (available from API version 5.52)
    /// - "wall" for wall posts
    /// - "wall_reply" for comments on wall posts
    /// - "sticker" for stickers
    /// - "gift" for gifts
    #[serde(rename = "type")]
    pub attachment_type: String,
    /// Depending on the `type`, this field contains an object representing the attachment.
    /// The structure of this object varies by `type`.
    pub item: AttachmentItem<T>,
}

/// Enum to represent all possible types of attachments in a personal message.
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AttachmentItem<T> {
    Photo(Photo),
    Video(Video),
    Audio(Audio),
    Doc(Doc),
    Link(Link<T>),
    Market(Product),
    Wall(Wall<T>),
    Sticker(Sticker),
    Gift(Gift),
}

/// Represents a photograph attachment.
#[derive(Debug, Deserialize, Clone)]
pub struct Photo {
    pub id: i64,
    pub album_id: i32,
    pub owner_id: i64,
    pub user_id: i32,
    pub text: String,
    pub date: i64,
    pub sizes: Vec<PhotoSize>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

/// Represents the size of a photo, including its URL and dimensions.
#[derive(Debug, Deserialize, Clone)]
pub struct PhotoSize {
    #[serde(rename = "type")]
    pub photo_type: String, // E.g., "s", "m", "x"
    pub url: String,
    pub width: i32,
    pub height: i32,
}

/// Represents a video attachment.
#[derive(Debug, Deserialize, Clone)]
pub struct Video {
    /// Identifier of the video.
    pub id: i32,
    /// Identifier of the video owner.
    pub owner_id: i32,
    /// Title of the video.
    pub title: String,
    /// Description of the video.
    pub description: String,
    /// Duration of the video in seconds.
    pub duration: i32,
    /// Cover image of the video. This is an array of objects, each with height, url, and width properties.
    pub image: Vec<VideoImage>,
    /// The first frame of the video. This is an array of objects, each with height, url, and width properties.
    pub first_frame: Vec<VideoImage>,
    /// Date when the video was added in Unixtime.
    pub date: i64,
    /// Date when the video was added by a user or group in Unixtime.
    pub adding_date: i64,
    /// Number of views of the video.
    pub views: i32,
    /// URL of the video page with the player.
    pub player: String,
    /// Access key of the video.
    pub access_key: Option<String>,
    /// Indicates if the video is private (`1` if private).
    pub is_private: Option<i32>,
    /// Width of the video.
    pub width: i32,
    /// Height of the video.
    pub height: i32,
    /// Indicates if the video can be added to favourites (`1` if can be added).
    pub can_add: i32,
}

/// Represents the cover image or first frame of a video.
#[derive(Debug, Deserialize, Clone)]
pub struct VideoImage {
    /// Height of the image.
    pub height: i32,
    /// URL of the image.
    pub url: String,
    /// Width of the image.
    pub width: i32,
}

/// Represents an audio attachment.
#[derive(Debug, Deserialize, Clone)]
pub struct Audio {
    /// Identifier of the audio recording.
    pub id: i32,
    /// Identifier of the audio owner.
    pub owner_id: i32,
    /// Artist of the audio recording.
    pub artist: String,
    /// Title of the audio recording.
    pub title: String,
    /// Duration of the audio in seconds.
    pub duration: i32,
    /// URL of the mp3 file.
    pub url: String,
    /// Identifier of the lyrics (if available).
    pub lyrics_id: Option<i32>,
    /// Identifier of the album (if assigned).
    pub album_id: Option<i32>,
    /// Genre identifier from the list of audio genres.
    pub genre_id: Option<i32>,
    /// Date the audio was added in Unixtime.
    pub date: i64,
    /// Indicates if the audio is in high quality (`1` if in high quality).
    pub is_hq: Option<i32>,
}

/// Represents a document attachment.
#[derive(Debug, Deserialize, Clone)]
pub struct Doc {
    /// Identifier of the document.
    pub id: i32,
    /// Identifier of the document owner.
    pub owner_id: i32,
    /// Title of the document.
    pub title: String,
    /// Size of the document in bytes.
    pub size: i64,
    /// Extension of the document.
    pub ext: String,
    /// URL of the document, where it can be downloaded.
    pub url: String,
    /// Date the document was added in Unixtime.
    pub date: i64,
    /// Type of the document. Possible values:
    /// - `1` for text documents;
    /// - `2` for archives;
    /// - `3` for gif;
    /// - `4` for images;
    /// - `5` for audio;
    /// - `6` for video;
    /// - `7` for e-books;
    /// - `8` for unknown.
    #[serde(rename = "type")]
    pub doc_type: i32,
    /// Access key of the document.
    pub access_key: Option<String>,
}

/// Represents an attached link in a message, comment, or wall post.
#[derive(Debug, Deserialize, Clone)]
pub struct Link<T> {
    /// URL of the link.
    pub url: String,
    /// Title of the link.
    pub title: String,
    /// Caption of the link, if available.
    pub caption: Option<String>,
    /// Description of the link.
    pub description: String,
    /// Image preview of the link, if available.
    pub photo: Option<Photo>,
    /// Information about the product, if available (e.g., for store links like AliExpress).
    pub product: Option<Product>,
    /// Information about the transition button, if available.
    pub button: Option<Button<T>>,
    /// Identifier of the wiki page with content preview. Format "owner_id_page_id".
    pub preview_page: String,
    /// URL of the page with content preview.
    pub preview_url: String,
}

/// Represents a product.
#[derive(Debug, Deserialize, Clone)]
pub struct Product {
    /// Product ID.
    pub id: i32,
    /// Owner ID of the product.
    pub owner_id: i32,
    /// Title of the product.
    pub title: String,
    /// Description of the product.
    pub description: String,
    /// Price object containing details about the product's price.
    pub price: Price,
    /// Product dimensions.
    pub dimensions: Option<Dimensions>,
    /// Weight of the product in grams.
    pub weight: Option<i32>,
    /// Product category.
    pub category: Category,
    /// URL of the product's cover image.
    pub thumb_photo: String,
    /// Creation date of the product in Unixtime.
    pub date: i64,
    /// Availability status of the product. Possible values: 0 (available), 1 (removed), 2 (unavailable).
    pub availability: i32,
    /// True if the product is added to favorites by the current user.
    pub is_favorite: bool,
    /// SKU of the product.
    pub sku: Option<String>,
}

/// Represents the price of a product.
#[derive(Debug, Deserialize, Clone)]
pub struct Price {
    /// Price of the product in hundredths of a currency unit.
    pub amount: String,
    /// Currency of the price.
    pub currency: Currency,
    /// Old price of the product in hundredths of a currency unit.
    pub old_amount: Option<String>,
    /// Textual representation of the price.
    pub text: String,
}

/// Represents currency details.
#[derive(Debug, Deserialize, Clone)]
pub struct Currency {
    /// Currency ID.
    pub id: i32,
    /// Currency symbol.
    pub name: String,
}

/// Represents product dimensions.
#[derive(Debug, Deserialize, Clone)]
pub struct Dimensions {
    /// Width in millimeters.
    pub width: i32,
    /// Height in millimeters.
    pub height: i32,
    /// Length in millimeters.
    pub length: i32,
}

/// Represents a product category.
#[derive(Debug, Deserialize, Clone)]
pub struct Category {
    /// Category ID.
    pub id: i32,
    /// Category name.
    pub name: String,
    /// Category section.
    pub section: Section,
}

/// Represents a section of a product category.
#[derive(Debug, Deserialize, Clone)]
pub struct Section {
    /// Section ID.
    pub id: i32,
    /// Section name.
    pub name: String,
}

/// Represents a sticker.
#[derive(Debug, Deserialize, Clone)]
pub struct Sticker {
    /// ID of the sticker pack.
    pub product_id: i32,
    /// Sticker ID.
    pub sticker_id: i32,
    /// Array of images for the sticker with a transparent background.
    pub images: Vec<StickerImage>,
    /// Array of images for the sticker with a non-transparent background.
    pub images_with_background: Vec<StickerImage>,
    /// URL of the sticker animation.
    pub animation_url: Option<String>,
    /// Information on whether the sticker is available.
    pub is_allowed: bool,
}

/// Represents an image of a sticker.
#[derive(Debug, Deserialize, Clone)]
pub struct StickerImage {
    /// URL of the image.
    pub url: String,
    /// Width of the image in px.
    pub width: i32,
    /// Height of the image in px.
    pub height: i32,
}

/// Represents a gift.
#[derive(Debug, Deserialize, Clone)]
pub struct Gift {
    /// Gift ID.
    pub id: i32,
    /// URL of the gift image (256x256 px).
    pub thumb_256: String,
    /// URL of the gift image (96x96 px).
    pub thumb_96: String,
    /// URL of the gift image (48x48 px).
    pub thumb_48: String,
}

/// Represents a wall post.
#[derive(Debug, Deserialize, Clone)]
pub struct Wall<T> {
    /// Identifier of the post.
    pub id: i32,
    /// Identifier of the wall owner.
    pub owner_id: i32,
    /// Identifier of the author of the post.
    pub from_id: i32,
    /// Identifier of the administrator who published the post (for community posts).
    pub created_by: Option<i32>,
    /// Time of publication in Unixtime.
    pub date: i64,
    /// Text of the post.
    pub text: String,
    /// Identifier of the owner of the post to which this post is a reply.
    pub reply_owner_id: Option<i32>,
    /// Identifier of the post to which this post is a reply.
    pub reply_post_id: Option<i32>,
    /// `1` if the post was created with the "Only for friends" option.
    pub friends_only: Option<i32>,
    /// Information about comments to the post.
    pub comments: Option<WallPostComments>,
    /// Information about likes to the post.
    pub likes: Option<WallPostLikes>,
    /// Information about reposts of the post ("Shares").
    pub reposts: Option<WallPostReposts>,
    /// Information about views of the post.
    pub views: Option<WallPostViews>,
    /// Type of the post.
    pub post_type: String,
    /// Information about attachments to the post (photos, links, etc.).
    pub attachments: Box<Option<Vec<Attachment<T>>>>,
    /// Geographical location information.
    pub geo: Option<Geo>,
    /// Identifier of the author (for community posts signed by a user).
    pub signer_id: Option<i32>,
    /// Copy history for reposts.
    pub copy_history: Box<Option<Vec<Wall<T>>>>,
    /// Indicates if the post can be pinned.
    pub can_pin: Option<i32>,
    /// Indicates if the post can be deleted.
    pub can_delete: Option<i32>,
    /// Indicates if the post can be edited.
    pub can_edit: Option<i32>,
    /// Indicates if the post is pinned.
    pub is_pinned: Option<i32>,
    /// Indicates if the post is marked as ads.
    pub marked_as_ads: Option<i32>,
    /// Indicates if the post is added to favorites.
    pub is_favorite: bool,
    /// Information about VK Donut.
    pub donut: Option<WallPostDonut>,
    /// Identifier of a postponed post.
    pub postponed_id: Option<i32>,
}

/// Represents comments information for a wall post.
#[derive(Debug, Deserialize, Clone)]
pub struct WallPostComments {
    pub count: i32,
    pub can_post: Option<i32>,
    pub groups_can_post: bool,
    pub can_close: bool,
    pub can_open: bool,
}

/// Represents likes information for a wall post.
#[derive(Debug, Deserialize, Clone)]
pub struct WallPostLikes {
    pub count: i32,
    pub user_likes: i32,
    pub can_like: i32,
    pub can_publish: i32,
}

/// Represents reposts information for a wall post.
#[derive(Debug, Deserialize, Clone)]
pub struct WallPostReposts {
    pub count: i32,
    pub user_reposted: i32,
}

/// Represents views information for a wall post.
#[derive(Debug, Deserialize, Clone)]
pub struct WallPostViews {
    pub count: i32,
}

/// Represents VK Donut information for a wall post.
#[derive(Debug, Deserialize, Clone)]
pub struct WallPostDonut {
    pub is_donut: bool,
    pub paid_duration: Option<i32>,
    pub can_publish_free_copy: bool,
    pub edit_mode: String,
}
