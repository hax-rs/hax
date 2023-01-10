use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta};

pub(crate) fn init(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(item as ItemFn);

    let init_fn = if let NestedMeta::Meta(meta) = &args[0] {
        if let Some(ident) = meta.path().get_ident() {
            ident
        } else {
            panic!("Expected ident");
        }
    } else {
        panic!("Expected meta");
    };

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
    let stmts = block.stmts;

    let expanded = quote! {
        #(#attrs)*
        #vis #unsafety #abi #constness fn #ident() {
            #init_fn();
            #(#stmts)*
        }
    };

    expanded.into()
}
