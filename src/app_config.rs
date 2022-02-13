use config::{Config, ConfigError, Environment};
use serde_derive::Deserialize;
use std::default::Default;

// AppConfig is a property of the "ui" struct
// Hence needs to implement Default
#[derive(Debug, Deserialize, Default, Clone)]
pub struct AppConfig {
  pub obs_path: String,
  pub obs_exe: String,
  pub obs_profile: String,
  pub disable_notifications: bool,
}

impl AppConfig {
  pub fn from_env() -> Result<AppConfig, ConfigError> {
    let mut c = Config::new();

    // These have to be lowercase even if
    // env var names are uppercase:
    c.set_default("obs_path", r"C:\Program Files\obs-studio\bin\64bit")?;
    c.set_default("obs_profile", "Recording")?;
    c.set_default("obs_exe", "obs64.exe")?;
    c.set_default("disable_notifications", "false")?;

    c.merge(Environment::default())?;

    c.try_into()
  }
}
