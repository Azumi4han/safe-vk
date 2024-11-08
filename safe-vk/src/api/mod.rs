use crate::RequestBuilder;
use std::fmt;
use std::sync::Arc;

mod bindings;
mod messages;
mod photos;
mod users;

pub use bindings::*;
pub use messages::*;
pub use photos::*;
pub use users::*;

pub struct MethodBuilder<T = ()> {
    request: Arc<RequestBuilder>,
    peer_id: Option<i64>,
    query: Vec<u8>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> MethodBuilder<T> {
    pub fn remove_peer_id(&mut self) {
        let key = String::from("peer_id=");
        let key_bytes = key.as_bytes();

        if let Some(start) = self
            .query
            .windows(key_bytes.len())
            .position(|window| window == key_bytes)
        {
            let end = self.query[start..]
                .iter()
                .position(|&byte| byte == b'&')
                .map(|pos| start + pos + 1)
                .expect("`Unable to remove peer_id`");

            self.query.drain(start..end);
        }
    }
}

impl std::ops::Deref for MethodBuilder {
    type Target = dyn CtxAbstraction;

    fn deref(&self) -> &Self::Target {
        self
    }
}

pub trait Write {
    fn write(&mut self, arg: &[u8]);

    fn write_fmt(&mut self, arg: impl fmt::Display) {
        self.write(arg.to_string().as_bytes())
    }

    #[inline]
    fn arg<T: WriteQuery>(&mut self, key: &str, value: T) -> &mut Self {
        key.write_query(self);
        self.write(b"=");
        value.write_query(self);
        self.write(b"&");
        self
    }

    #[inline]
    fn arg_fmt(&mut self, key: &str, value: impl fmt::Display) -> &mut Self {
        key.write_query(self);
        self.write(b"=");
        self.write_fmt(value);
        self.write(b"&");
        self
    }

    #[inline]
    fn arg_json<T: serde::Serialize>(&mut self, key: &str, value: T) -> &mut Self {
        let json = serde_json::to_string(&value).expect("Invalid JSON");
        key.write_query(self);
        self.write(b"=");
        json.write_query(self);
        self.write(b"&");
        self
    }
}

pub trait WriteQuery: Sized {
    fn write_query<W>(&self, out: &mut W)
    where
        W: Write + ?Sized;
}

impl<'a> WriteQuery for &'a str {
    fn write_query<W>(&self, out: &mut W)
    where
        W: Write + ?Sized,
    {
        out.write(self.as_bytes())
    }
}

impl WriteQuery for String {
    fn write_query<W>(&self, out: &mut W)
    where
        W: Write + ?Sized,
    {
        out.write(self.as_bytes())
    }
}

impl WriteQuery for bool {
    fn write_query<W>(&self, out: &mut W)
    where
        W: Write + ?Sized,
    {
        let byte = if *self { 1u8 } else { 0u8 };
        out.write(&[byte])
    }
}

macro_rules! write_query_slice {
    ($ty:ty) => {
        impl WriteQuery for $ty {
            fn write_query<W>(&self, out: &mut W)
            where
                W: Write + ?Sized,
            {
                let ids = self
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                out.write(ids.as_bytes())
            }
        }
    };
}

write_query_slice!(&[u32]);
write_query_slice!(&[i32]);
write_query_slice!(&[i64]);

macro_rules! write_query {
    ($ty:ty) => {
        impl WriteQuery for $ty {
            fn write_query<W>(&self, out: &mut W)
            where
                W: Write + ?Sized,
            {
                let mut buf = ::itoa::Buffer::new();
                let s = buf.format(*self);
                out.write(s.as_bytes())
            }
        }
    };
}

write_query!(i64);
write_query!(i32);
write_query!(u32);
write_query!(u16);
write_query!(i8);
write_query!(usize);

/// Private API
#[macro_export]
#[doc(hidden)]
macro_rules! __method {
    (
        $(
            $(#[$doc:meta])*
            fn $fn_name:ident($( $arg:ident: $ty:ty ),*)
        )*
    ) => {
        $(
            $(#[$doc])*
            pub fn $fn_name(mut self, $($arg: $ty),*) -> Self {
                self.arg(stringify!($fn_name), $($arg),*);
                self
            }
        )*
    };
}

/// Private API
#[doc(hidden)]
#[macro_export]
macro_rules! _define_abstraction {
    (
        $trait_name:ident for $builder:ty {
            $(
                $(#[$meta:meta])*
                fn $method_name:ident $( ( $($arg_name:ident : $arg_type:ty),* ) )?
                -> $ret:ty {
                    peer_id: $include_peer_id:expr
                };
            )*
        }
    ) => {
        pub trait $trait_name {
            fn new(request: Arc<RequestBuilder>, peer_id: Option<i64>) -> $builder where Self: Sized;

            $(
                $(#[$meta])*
                fn $method_name(&self) -> $ret;
            )*
        }

        impl $trait_name for $builder {
            fn new(request: Arc<RequestBuilder>, peer_id: Option<i64>) -> Self {
                Self {
                    request,
                    peer_id,
                    query: vec![],
                    _marker: std::marker::PhantomData,
                }
            }

            $(
                $(#[$meta])*
                fn $method_name(&self) -> $ret {
                    let mut method = MethodBuilder {
                        request: self.request.clone(),
                        peer_id: self.peer_id,
                        query: vec![],
                        _marker: std::marker::PhantomData,
                    };

                    if $include_peer_id {
                        if let Some(peer_id) = self.peer_id {
                            method.arg("peer_id", peer_id);
                        }
                    }

                    method
                }
            )*
        }
    };
}
