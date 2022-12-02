mod entry;
mod init;

use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    entry::main(attr, item)
}

#[proc_macro_attribute]
pub fn init(attr: TokenStream, item: TokenStream) -> TokenStream {
    init::init(attr, item)
}
