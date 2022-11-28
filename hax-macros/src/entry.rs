use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub(crate) fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // let attr = parse_macro_input!(attr as AttributeArgs);
    // let item = parse_macro_input!(item as Item);

    let ast = parse_macro_input!(item as ItemFn);

    let stmts = &ast.block.stmts;

    let expanded = quote! {
        #[cfg(feature = "external")]
        fn main() {
            unimplemented!();

            #(#stmts)*
        }

        #[cfg(feature = "internal")]
        #[link(name = "kernel32")]
        extern "system" {
            fn FreeLibraryAndExitThread(module: usize, exit_code: u32) -> !;
        }

        #[cfg(feature = "internal")]
        #[no_mangle]
        unsafe extern "system" fn DllMain(module: usize, reason: u32, _: usize) -> isize {
            if reason == 1 {
                std::thread::spawn(move || unsafe {
                    #(#stmts)*

                    FreeLibraryAndExitThread(module as _, 0);
                });

                return 1;
            }

            0
        }
    };

    expanded.into()
}
