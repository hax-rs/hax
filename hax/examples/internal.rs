use std::thread;
use std::time::Duration;

#[link(name = "kernel32")]
extern "system" {
    fn AllocConsole() -> isize;
    fn FreeConsole() -> isize;
}

#[hax::main]
fn entrypoint() {
    unsafe {
        AllocConsole();
    }

    println!("Hello, world!");

    thread::sleep(Duration::new(3, 0));

    unsafe {
        FreeConsole();
    }
}
