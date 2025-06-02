use crate::config::WebhookConfig;
use chrono::{Local, DateTime, Duration};
use log::error;
use serde_json::json;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::VecDeque;

// Store message queue and last sent time
struct MessageQueue {
    last_sent: Option<DateTime<Local>>,
    messages: VecDeque<(String, String)>,  // (status, message)
}

static MESSAGE_QUEUE: Lazy<Mutex<MessageQueue>> = Lazy::new(|| {
    Mutex::new(MessageQueue {
        last_sent: None,
        messages: VecDeque::new(),
    })
});

pub async fn send_webhook(config: &WebhookConfig, status: &str, message: &str) {
    let now = Local::now();
    let mut queue = MESSAGE_QUEUE.lock().unwrap();
    
    // Add new message to queue
    queue.messages.push_back((status.to_string(), message.to_string()));
    
    // Check if we should send messages
    let should_send = match queue.last_sent {
        Some(last_sent) => {
            let duration = now.signed_duration_since(last_sent);
            duration >= Duration::seconds(config.message_interval as i64)
        }
        None => true,  // If this is the first message, send immediately
    };
    
    if should_send {
        // Merge all messages in queue
        let mut content = String::new();
        while let Some((status, msg)) = queue.messages.pop_front() {
            content.push_str(&format!("[{}] {}\n", status, msg));
        }
        
        // Send merged message
        let client = reqwest::Client::new();
        let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        
        // Check if URL is empty
        if config.url.is_empty() {
            error!("Webhook URL is empty");
            return;
        }
        
        log::info!("Sending webhook to URL: {}", config.url);
        
        let payload = json!({
            "msgtype": "text",
            "text": {
                "content": format!("{} [{}]\n{}", config.prefix, now_str, content)
            }
        });

        match client.post(&config.url)
            .json(&payload)
            .send()
            .await {
            Ok(_) => {
                log::info!("Webhook sent successfully");
                queue.last_sent = Some(now);
            }
            Err(e) => error!("Failed to send webhook: {}", e),
        }
    }
} 