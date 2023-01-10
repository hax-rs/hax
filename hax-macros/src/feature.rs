use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_feature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let ident = &ast.ident;
    let name = ident.to_string();
    let expanded = quote! {
        #[doc(hidden)]
        #[hax::init_fn(hax::FEATURES_INIT)]
        fn feature_init() -> hax::FeatureWrapper {
            hax::FeatureWrapper::new(#name, 0, Box::new(#ident::default()))
        }
    };

    expanded.into()
}
