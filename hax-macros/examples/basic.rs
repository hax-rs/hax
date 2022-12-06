use hax::Memory;

fn setup_hooks() {
    println!("setup_hooks");
}

#[hax_macros::main(process = "ac_client")]
#[hax_macros::init(setup_hooks)]
fn main(memory: Memory) {
    println!("Hello, world!");
}
