use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

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
