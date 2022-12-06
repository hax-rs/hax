use linkme::distributed_slice;

#[distributed_slice]
pub static FEATURES_INIT: [fn() -> Box<dyn Feature>] = [..];

pub type FeatureFn = Box<fn() -> FeatureBox>;
pub type FeatureBox = Box<dyn Feature>;

pub trait Feature {
    fn new() -> Self
    where
        Self: Sized;

    fn setup(&mut self);

    fn tick(&mut self);

    fn cleanup(&mut self);
}

pub struct FeatureWrapper {
    pub name: String,
    pub enabled: bool,
    pub key: i16,
    pub imp: FeatureFn,
}

impl FeatureWrapper {
    pub const fn new(name: String, key: i16, imp: FeatureFn) -> Self {
        Self {
            name,
            enabled: false,
            key,
            imp,
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}
