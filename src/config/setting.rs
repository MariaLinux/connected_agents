use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
  pub plugin_settings: Vec<PluginSettings>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginSettings {
  pub name: String,
  pub enabled: bool,
}