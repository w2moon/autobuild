use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub prefix: String,
    pub message_interval: u64,  // 消息聚合时间间隔（秒）
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
                message_interval: 60,  // 默认60秒
            },
        }
    }
}

pub fn load_config(config_path: Option<&str>) -> anyhow::Result<Config> {
    let config = if let Some(path) = config_path {
        log::info!("Loading config from specified file: {}", path);
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        log::info!("Loaded config: {:?}", config);
        config
    } else {
        // 尝试加载当前目录下的 autobuild.json
        let default_path = "autobuild.json";
        if std::path::Path::new(default_path).exists() {
            log::info!("Loading config from default file: {}", default_path);
            let content = std::fs::read_to_string(default_path)?;
            let config: Config = serde_json::from_str(&content)?;
            log::info!("Loaded config: {:?}", config);
            config
        } else {
            log::info!("No config file found, using default config");
            Config::default()
        }
    };
    Ok(config)
}