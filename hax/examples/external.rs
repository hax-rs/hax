pub mod feature;

#[hax::main]
fn main() {
    let mut features = hax::FEATURES_INIT
        .iter()
        .map(|f| f())
        .collect::<Vec<Box<dyn hax::Feature>>>();

    for f in features.iter_mut() {
        f.setup();
        f.tick();
        f.cleanup();
    }
}
