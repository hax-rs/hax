use std::ops::{Deref, DerefMut};

use linkme::distributed_slice;
use serde::{Deserialize, Serialize};

#[distributed_slice]
pub static FEATURES_INIT: [fn() -> FeatureWrapper] = [..];

pub type FeatureBox = Box<dyn FeatureTrait>;

/// Returns a list of all the features.
pub fn features() -> Vec<FeatureWrapper> {
    FEATURES_INIT.iter().map(|f| f()).collect::<Vec<_>>()
}

#[typetag::serde]
pub trait FeatureTrait {
    fn new() -> Self
    where
        Self: Sized + Default,
    {
        Default::default()
    }

    fn setup(&mut self);

    fn tick(&mut self);

    fn cleanup(&mut self);
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

    /// Load config from `config.toml`.
    pub fn load(&mut self) {
        if let Ok(config) = std::fs::read_to_string("config.toml") {
            if let Ok(config) = toml::from_str::<Vec<Self>>(&config) {
                if let Some(config) = config.into_iter().find(|c| c.name == self.name) {
                    self.enabled = config.enabled;
                    self.key = config.key;
                    self.feature = config.feature;
                }
            }
        }
    }

    /// Save config to `config.toml`.
    pub fn save(&self) {
        let toml = toml::to_string_pretty(self).unwrap();

        use std::io::Write;
        if let Ok(mut file) = std::fs::OpenOptions::new().append(true).open("config.toml") {
            if let Err(e) = writeln!(file, "{}", toml) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
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
