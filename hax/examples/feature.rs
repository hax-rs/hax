use hax::{Feature, FeatureBox};

// #[derive(Feature)]
#[derive(Debug, Default)]
pub struct ExampleFeature {
    pub foo: u32,
}

impl Feature for ExampleFeature {
    fn new() -> Self {
        println!("ExampleFeature::new");
        Default::default()
    }

    fn setup(&mut self) {
        println!("ExampleFeature::setup");
    }

    fn tick(&mut self) {
        println!("ExampleFeature::tick");
    }

    fn cleanup(&mut self) {
        println!("ExampleFeature::cleanup");
    }
}

#[hax::init_fn(hax::FEATURES_INIT)]
fn feature_init() -> FeatureBox {
    Box::new(ExampleFeature::new())
}

#[allow(unused)]
fn main() {}
