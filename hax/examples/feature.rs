use hax::{Feature, FeatureTrait};
use serde::{Deserialize, Serialize};

#[derive(Feature, Debug, Default, Serialize, Deserialize)]
pub struct ExampleFeature {
    pub foo: u32,

    // #[serde(skip)]
    pub bar: u32,
}

#[hax::typetag]
impl FeatureTrait for ExampleFeature {
    fn setup(&mut self) {
        log::info!("Feature: {:?}", self);
        log::info!("ExampleFeature::setup");
    }

    fn tick(&mut self) {
        log::info!("ExampleFeature::tick");
    }

    fn cleanup(&mut self) {
        log::info!("ExampleFeature::cleanup");
    }
}

#[allow(unused)]
fn main() {}

#[test]
fn test() {
    let mut feature = feature_init();

    feature.load();
    feature.save();
}
