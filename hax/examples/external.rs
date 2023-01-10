pub mod feature;

#[hax::main]
fn main() {
    let mut features = hax::features();

    // TODO: How to load/save config?
    // - Default: load from config.toml
    // TODO: How to pass overlay?
    // TODO: How to handle keyboard input?

    features.iter_mut().for_each(|f| f.setup());
    for f in features.iter_mut() {
        f.tick();
    }
    features.iter_mut().for_each(|f| f.cleanup());
}
