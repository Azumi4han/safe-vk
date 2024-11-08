use crate::{
    __method,
    api::{GetUsersMethod, MethodBuilder, Write},
};

impl MethodBuilder<GetUsersMethod> {
    __method! {
        fn user_ids(ids: &[i32])
        fn fields(name: &str)
        fn name_case(name: &str)
        fn from_group_id(group_id: u32)
    }
}
