fn setup_hooks() {
    println!("setup_hooks");
}

#[hax_macros::main(init = "setup_hooks")]
fn test() {
    println!("Hello, world!");
}
