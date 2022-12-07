use hax::{Feature, FeatureWrapper};
use serde::{Deserialize, Serialize};

// #[derive(Feature)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExampleFeature {
    pub foo: u32,
}

#[typetag::serde]
impl Feature for ExampleFeature {
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
fn feature_init() -> FeatureWrapper {
    FeatureWrapper::new("ExampleFeature", 0, Box::new(ExampleFeature::new()))
}

#[allow(unused)]
fn main() {}

#[test]
fn test() {
    let mut feature = feature_init();

    feature.load();
    feature.save();
}
