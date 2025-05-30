use crate::config::WebhookConfig;
use chrono::Local;
use log::error;
use serde_json::json;

pub async fn send_webhook(config: &WebhookConfig, status: &str, message: &str) {
    let client = reqwest::Client::new();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let payload = json!({
        "msgtype": "text",
        "text": {
            "content": format!("{} [{}] {}\n{}", config.prefix, now, status, message)
        }
    });

    match client.post(&config.url)
        .json(&payload)
        .send()
        .await {
        Ok(_) => (),
        Err(e) => error!("Failed to send webhook: {}", e),
    }
} 