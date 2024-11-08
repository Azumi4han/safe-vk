use crate::{
    RequestBuilder, _define_abstraction,
    api::{GetUsersMethod, MethodBuilder, Write},
};
use std::sync::Arc;

_define_abstraction! {
    AbstractionUsers for MethodBuilder {
        // Returns enhanced user information.
        fn get -> MethodBuilder<GetUsersMethod> {
            peer_id: true
        };
    }
}
