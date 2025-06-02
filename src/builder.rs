use std::process::Command;
use anyhow::Result;
use log::{info, error};
use std::time::Instant;
use std::sync::Mutex;
use std::process::Child;
use once_cell::sync::Lazy;

static CURRENT_PROCESS: Lazy<Mutex<Option<Child>>> = Lazy::new(|| Mutex::new(None));

pub async fn execute_command(command: &str, webhook: &crate::config::WebhookConfig) -> Result<()> {
    let start = Instant::now();
    
    // 取消当前正在执行的命令
    if let Some(mut current_process) = CURRENT_PROCESS.lock().unwrap().take() {
        let _ = current_process.kill();
        let cancel_msg = "上一个命令已被取消";
        info!("{}", cancel_msg);
        crate::webhook::send_webhook(webhook, "INFO", cancel_msg).await;
    }

    let status = format!("开始执行命令: {}", command);
    crate::webhook::send_webhook(webhook, "INFO", &status).await;

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()?;

    // 存储新的进程
    let mut current_process = CURRENT_PROCESS.lock().unwrap();
    *current_process = Some(child);
    drop(current_process);

    let output = CURRENT_PROCESS.lock().unwrap().take().unwrap().wait_with_output()?;

    // 清除已完成的进程
    *CURRENT_PROCESS.lock().unwrap() = None;

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