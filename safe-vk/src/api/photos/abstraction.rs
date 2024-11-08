use crate::{
    RequestBuilder, _define_abstraction,
    api::{GetMessagesUploadServerMethod, MethodBuilder, SaveMessagesPhotoMethod, Write},
};
use std::sync::Arc;

_define_abstraction! {
    AbstractionPhotos for MethodBuilder {
        // Get server link for uploading attachment
        fn get_messages_upload_server -> MethodBuilder<GetMessagesUploadServerMethod> {
            peer_id: false
        };

        fn save_messages_photo -> MethodBuilder<SaveMessagesPhotoMethod> {
            peer_id: false
        };
    }
}
