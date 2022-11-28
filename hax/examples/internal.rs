extern crate hax;

#[link(name = "kernel32")]
extern "system" {
    fn AllocConsole() -> isize;
    fn FreeConsole() -> isize;
}

#[hax::main]
fn hax() {
    AllocConsole();

    println!("Hello, world!");

    std::thread::sleep(std::time::Duration::new(5, 0));

    FreeConsole();
}
