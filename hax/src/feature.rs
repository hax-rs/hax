use std::ops::{Deref, DerefMut};

use linkme::distributed_slice;

#[distributed_slice]
pub static FEATURES_INIT: [fn() -> FeatureWrapper] = [..];

pub type FeatureBox = Box<dyn Feature>;

pub trait Feature {
    fn new() -> Self
    where
        Self: Sized + Default;

    fn setup(&mut self);

    fn tick(&mut self);

    fn cleanup(&mut self);
}

pub struct FeatureWrapper {
    pub name: String,
    pub enabled: bool,
    pub key: i16,
    pub feature: FeatureBox,
}

impl FeatureWrapper {
    pub fn new(name: impl ToString, key: i16, feature: FeatureBox) -> Self {
        Self {
            name: name.to_string(),
            enabled: false,
            key,
            feature,
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

impl Deref for FeatureWrapper {
    type Target = FeatureBox;

    fn deref(&self) -> &Self::Target {
        &self.feature
    }
}

impl DerefMut for FeatureWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.feature
    }
}
