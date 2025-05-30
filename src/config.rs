use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub prefix: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub repository: PathBuf,
    pub build: String,
    pub publish: String,
    pub branch: String,
    pub interval: u64,
    pub webhook: WebhookConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            repository: PathBuf::from("."),
            build: "npm run build".to_string(),
            publish: "npm run publish".to_string(),
            branch: "main".to_string(),
            interval: 10,
            webhook: WebhookConfig {
                url: "".to_string(),
                prefix: "".to_string(),
            },
        }
    }
}

pub fn load_config(config_path: Option<&str>) -> anyhow::Result<Config> {
    let config = if let Some(path) = config_path {
        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content)?
    } else {
        Config::default()
    };
    Ok(config)
}