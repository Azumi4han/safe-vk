use crate::{
    __method,
    api::{MethodBuilder, SaveMessagesPhotoMethod, Write},
};

impl MethodBuilder<SaveMessagesPhotoMethod> {
    __method! {
        fn server(id: i32)
        fn hash(hash: &str)
        fn photo(info: &str)
    }
}
