use std::fmt;

pub mod messages;
pub mod photos;
pub mod users;

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

////TODO: Macro
impl WriteQuery for &[i64] {
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

impl WriteQuery for &[i32] {
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

impl WriteQuery for &[u32] {
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
////

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

#[macro_export]
macro_rules! api_func {
    ($fn_name:ident, $ty:ty) => {
        pub fn $fn_name(mut self, value: $ty) -> Self {
            self.arg(stringify!($fn_name), value);
            self
        }
    };
}

#[macro_export]
macro_rules! chained_method_fn {
    ($name:ident, Option<$output:ty>, $method:expr, $($fn_name:ident($ty:ty)),*) => {
        #[derive(Clone)]
        pub struct $name {
            request: Arc<RequestBuilder>,
            query: Vec<u8>,
        }

        impl Write for $name {
            fn write(&mut self, arg: &[u8]) {
                self.query.extend_from_slice(arg);
            }

            fn write_fmt(&mut self, arg: impl fmt::Display) {
                use std::io::Write;
                write!(self.query, "{arg}").unwrap();
            }
        }

        impl $name {
            pub fn new(request: Arc<RequestBuilder>, peer_id: Option<i64>) -> Self {
                let mut method = $name { request, query: Vec::new() };
                if let Some(peer_id) = peer_id { method.arg("peer_id", peer_id); };
                method
            }

            $(api_func!($fn_name, $ty);)*
        }

        impl IntoFuture for $name {
            type Output = Result<Option<$output>>;
            type IntoFuture = futures_core::future::BoxFuture<'static, Result<Option<$output>>>;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    let response = self.request.post(VK, $method, &self.query, {}).await?;
                    let parsed = parse_response!(response, Option<$output>)?;
                    Ok(parsed)
                })
            }
        }
    };
    ($name:ident, $output:ty, $method:expr, $($fn_name:ident($ty:ty)),*) => {
        pub struct $name {
            request: Arc<RequestBuilder>,
            query: Vec<u8>,
        }

        impl Write for $name {
            fn write(&mut self, arg: &[u8]) {
                self.query.extend_from_slice(arg);
            }

            fn write_fmt(&mut self, arg: impl fmt::Display) {
                use std::io::Write;
                write!(self.query, "{arg}").unwrap();
            }
        }

        impl $name {
            pub fn new(request: Arc<RequestBuilder>, peer_id: Option<i64>) -> Self {
                let mut method = $name { request, query: Vec::new() };
                if let Some(peer_id) = peer_id { method.arg("peer_id", peer_id); };
                method
            }

            $(api_func!($fn_name, $ty);)*
        }

        impl IntoFuture for $name {
            type Output = Result<$output>;
            type IntoFuture = futures_core::future::BoxFuture<'static, Result<$output>>;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    let response = self.request.post(VK, $method, &self.query, {}).await?;
                    let parsed = parse_response!(response, $output)?;
                    Ok(parsed)
                })
            }
        }
    };
}
