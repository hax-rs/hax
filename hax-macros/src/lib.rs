mod entry;
mod feature;
mod init;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    entry::main(attr, item)
}

#[proc_macro_attribute]
pub fn init(attr: TokenStream, item: TokenStream) -> TokenStream {
    init::init(attr, item)
}

#[proc_macro_derive(Feature)]
pub fn derive_feature(item: TokenStream) -> TokenStream {
    feature::derive_feature(item)
}
