use std::thread;
use std::time::Duration;

hax::main!(entrypoint);

#[link(name = "kernel32")]
extern "system" {
    fn AllocConsole() -> isize;
    fn FreeConsole() -> isize;
}

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
