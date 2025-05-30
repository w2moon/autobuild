use crate::config::WebhookConfig;
use chrono::{Local, DateTime, Duration};
use log::error;
use serde_json::json;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::VecDeque;

// 存储待发送的消息队列和上次发送时间
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
    
    // 将新消息添加到队列
    queue.messages.push_back((status.to_string(), message.to_string()));
    
    // 检查是否需要发送消息
    let should_send = match queue.last_sent {
        Some(last_sent) => {
            let duration = now.signed_duration_since(last_sent);
            duration >= Duration::seconds(config.message_interval as i64)
        }
        None => true,  // 如果是第一条消息，立即发送
    };
    
    if should_send {
        // 合并队列中的所有消息
        let mut content = String::new();
        while let Some((status, msg)) = queue.messages.pop_front() {
            content.push_str(&format!("[{}] {}\n", status, msg));
        }
        
        // 发送合并后的消息
        let client = reqwest::Client::new();
        let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        
        // 检查 URL 是否为空
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