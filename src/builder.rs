use std::process::Command;
use anyhow::Result;
use log::{info, error};
use std::time::Instant;

pub async fn execute_command(command: &str, webhook: &crate::config::WebhookConfig) -> Result<()> {
    let start = Instant::now();
    let status = format!("开始执行命令: {}", command);
    crate::webhook::send_webhook(webhook, "INFO", &status).await;

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    if output.status.success() {
        let duration = start.elapsed();
        let success_msg = format!(
            "命令执行成功: {}\n耗时: {:.2}秒\n输出:\n{}",
            command,
            duration.as_secs_f64(),
            String::from_utf8_lossy(&output.stdout)
        );
        info!("{}", success_msg);
        crate::webhook::send_webhook(webhook, "SUCCESS", &success_msg).await;
        Ok(())
    } else {
        let error_msg = format!(
            "命令执行失败: {}\n错误:\n{}",
            command,
            String::from_utf8_lossy(&output.stderr)
        );
        error!("{}", error_msg);
        crate::webhook::send_webhook(webhook, "ERROR", &error_msg).await;
        Err(anyhow::anyhow!(error_msg))
    }
} 