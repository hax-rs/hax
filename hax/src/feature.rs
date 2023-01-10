use std::ops::{Deref, DerefMut};

use linkme::distributed_slice;
use serde::{Deserialize, Serialize};

#[distributed_slice]
pub static FEATURES_INIT: [fn() -> FeatureWrapper] = [..];

pub type FeatureBox = Box<dyn FeatureTrait>;

/// Returns a list of all the features.
pub fn features() -> Vec<FeatureWrapper> {
    let mut features = FEATURES_INIT.iter().map(|f| f()).collect::<Vec<_>>();

    for feature in &mut features {
        feature.load();
    }

    features
}

#[typetag::serde]
pub trait FeatureTrait {
    fn new() -> Self
    where
        Self: Sized + Default,
    {
        Default::default()
    }

    fn setup(&mut self) {}

    fn tick(&mut self) {}

    fn cleanup(&mut self) {}
}

#[derive(Serialize, Deserialize)]
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

    /// Load config from `config_{name}.toml`.
    pub fn load(&mut self) {
        log::debug!("Loading feature from disk");

        let file = format!("config_{}.toml", self.name);
        let Ok(config) = std::fs::read_to_string(file) else {
            log::warn!("Couldn't find config.toml");
            return;
        };
        let config = match toml::from_str::<Self>(&config) {
            Ok(config) => config,
            Err(error) => {
                log::warn!("Couldn't parse config.toml: {}", error);
                return;
            }
        };

        self.enabled = config.enabled;
        self.key = config.key;
        self.feature = config.feature;
    }

    /// Save config to `config.toml`.
    pub fn save(&self) {
        log::debug!("Saving feature to disk");

        let toml = toml::to_string(self).unwrap();
        let file = format!("config_{}.toml", self.name);
        std::fs::write(file, toml).unwrap();
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
