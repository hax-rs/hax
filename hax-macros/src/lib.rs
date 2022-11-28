mod entry;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    entry::main(attr, item)
}
