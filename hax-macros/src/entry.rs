use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

#[derive(Debug, FromMeta)]
pub struct MacroArgs {
    #[darling(default)]
    process: Option<String>,
}

pub(crate) fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(item as ItemFn);

    let args = match MacroArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    let _ = args;

    let ItemFn {
        attrs,
        block,
        vis,
        sig:
            syn::Signature {
                ident,
                unsafety,
                constness,
                abi,
                ..
            },
        ..
    } = item;

    let expanded = quote! {
        #(#attrs)*
        #vis #unsafety #abi #constness fn #ident() #block
    };

    expanded.into()
}
