use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, ItemFn, LitStr, ReturnType};

#[proc_macro_attribute]
pub fn auto_ok(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    if input_fn.sig.asyncness.is_some() {
        if matches!(input_fn.sig.output, ReturnType::Default) {
            input_fn.sig.output = syn::parse_quote!(-> safe_vk::Result<()>);
        }

        let ok_stmt: syn::Stmt = syn::parse_quote! {
            return Ok(());
        };

        input_fn.block.stmts.push(ok_stmt);
    }

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input_fn;

    let expanded = quote! {
        #(#attrs)* #vis #sig {
            #block
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Method, attributes(method_path, optional))]
pub fn derive_method(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let method_struct_name = format_ident!("{}Method", struct_name);
    let response_struct_name = format_ident!("{}Response", struct_name);

    let is_optional = input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("optional"));

    let method_path = input
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("method_path") {
                Some(
                    attr.parse_args::<LitStr>()
                        .expect("Expected a string literal"),
                )
            } else {
                None
            }
        })
        .expect("Expected #[method_path = \"..\"] attribute");

    let response_type = if is_optional {
        quote! { Option<#response_struct_name> }
    } else {
        quote! { #response_struct_name }
    };

    let generated = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            // Handle named fields (normal struct with field names)
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|f| &f.ident);
                let field_types = fields.named.iter().map(|f| &f.ty);

                quote! {
                    pub struct #method_struct_name;

                    #[derive(serde::Deserialize, Debug)]
                    pub struct #response_struct_name {
                        // Add `pub` to the field definitions here
                        #(pub #field_names: #field_types),*
                    }

                    impl crate::api::Write for crate::api::MethodBuilder<#method_struct_name> {
                        fn write(&mut self, arg: &[u8]) {
                            self.query.extend_from_slice(arg);
                        }

                        fn write_fmt(&mut self, arg: impl std::fmt::Display) {
                            use std::io::Write;
                            write!(self.query, "{}", arg).unwrap();
                        }
                    }

                    impl IntoFuture for crate::api::MethodBuilder<#method_struct_name> {
                        type Output = crate::Result<#response_type>;
                        type IntoFuture = futures_core::future::BoxFuture<'static, crate::Result<#response_type>>;

                        fn into_future(self) -> Self::IntoFuture {
                            Box::pin(async move {
                                let response = self.request.post(crate::VK, #method_path, &self.query, {}).await?;
                                let parsed = parse_response!(response, #response_type)?;
                                Ok(parsed)
                            })
                        }
                    }
                }
            }
            // Handle unnamed fields (tuple structs)
            Fields::Unnamed(fields) => {
                let field_types = fields.unnamed.iter().map(|f| &f.ty).collect::<Vec<_>>();

                // Use the single type in the tuple
                let field_type = &field_types[0];

                quote! {
                        pub struct #method_struct_name;

                        impl crate::api::Write for crate::api::MethodBuilder<#method_struct_name> {
                            fn write(&mut self, arg: &[u8]) {
                                self.query.extend_from_slice(arg);
                            }

                            fn write_fmt(&mut self, arg: impl std::fmt::Display) {
                                use std::io::Write;
                                write!(self.query, "{}", arg).unwrap();
                            }
                        }

                        impl IntoFuture for crate::api::MethodBuilder<#method_struct_name> {
                            type Output = crate::Result<#field_type>;
                            type IntoFuture = futures_core::future::BoxFuture<'static, crate::Result<#field_type>>;

                            fn into_future(self) -> Self::IntoFuture {
                                Box::pin(async move {
                                    let response = self.request.post(crate::VK, #method_path, &self.query, {}).await?;
                                    let parsed = parse_response!(response, #field_type)?;
                                    Ok(parsed)
                                })
                            }
                        }

                }
            }
            // Handle unit structs (no fields)
            Fields::Unit => {
                quote! {
                    pub struct #method_struct_name;

                    #[derive(serde::Deserialize, Debug)]
                    pub struct #response_struct_name;

                    impl crate::api::Write for crate::api::MethodBuilder<#method_struct_name> {
                        fn write(&mut self, arg: &[u8]) {
                            self.query.extend_from_slice(arg);
                        }

                        fn write_fmt(&mut self, arg: impl std::fmt::Display) {
                            use std::io::Write;
                            write!(self.query, "{}", arg).unwrap();
                        }
                    }

                    impl IntoFuture for crate::api::MethodBuilder<#method_struct_name> {
                        type Output = crate::Result<#response_type>;
                        type IntoFuture = futures_core::future::BoxFuture<'static, crate::Result<#response_type>>;

                        fn into_future(self) -> Self::IntoFuture {
                            Box::pin(async move {
                                let response = self.request.post(crate::VK, #method_path, &self.query, {}).await?;
                                let parsed = parse_response!(response, #response_type)?;
                                Ok(parsed)
                            })
                        }
                    }
                }
            }
        },
        _ => quote!(),
    };

    generated.into()
}
