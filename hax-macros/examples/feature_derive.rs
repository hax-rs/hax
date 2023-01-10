use hax::FeatureTrait;
use hax_macros::Feature;
use serde::{Deserialize, Serialize};

#[derive(Feature, Serialize, Deserialize, Default)]
pub struct ExampleFeature {
    foo: u32,
}

#[hax::typetag]
impl FeatureTrait for ExampleFeature {
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

fn main() {
    let features = hax::features();
    assert_eq!(features.len(), 1);
}
