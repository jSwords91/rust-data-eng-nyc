use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub anomaly_detection: AnomalyDetectionSettings,
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub data_url: String,
    pub output_file: String,
}

#[derive(Debug, Deserialize)]
pub struct AnomalyDetectionSettings {
    pub iqr_multiplier: f64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .build()?;

        s.try_deserialize()
    }
}