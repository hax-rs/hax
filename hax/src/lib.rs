pub use hax_macros::*;

// #[cfg(feature = "external")]
// #[macro_export]
// macro_rules! main {
//     ($function:expr) => {
//         fn main() {
//             $function();
//         }
//     };
// }

// #[cfg(feature = "internal")]
// #[macro_export]
// macro_rules! main {
//     ($function:expr) => {
//         #[link(name = "kernel32")]
//         extern "system" {
//             fn FreeLibraryAndExitThread(module: usize, exit_code: u32) -> !;
//         }

//         #[no_mangle]
//         unsafe extern "system" fn DllMain(module: usize, reason: u32, _: usize) -> bool {
//             if reason == 1 {
//                 std::thread::spawn(move || unsafe {
//                     $function();

//                     FreeLibraryAndExitThread(module, 0);
//                 });

//                 return true;
//             }

//             false
//         }
//     };
// }
