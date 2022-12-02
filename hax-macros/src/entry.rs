use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

#[derive(Debug, FromMeta)]
pub struct MacroArgs {
    #[darling(default)]
    init: Option<String>,
}

pub(crate) fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let ast = parse_macro_input!(item as ItemFn);

    let args = match MacroArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let init_call = if let Some(ref init) = args.init {
        let ident = Ident::new(init, Span::call_site());
        quote! { #ident(); }
    } else {
        quote! {}
    };

    let stmts = &ast.block.stmts;
    let expanded = quote! {
        fn main() {
            #init_call
            #(#stmts)*
        }
    };

    expanded.into()
}
