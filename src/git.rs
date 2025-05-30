use std::process::Command;
use anyhow::Result;
use log::{info, error};
use crate::config::WebhookConfig;

pub async fn check_and_pull(repo_path: &str, branch: &str, webhook: &WebhookConfig) -> Result<bool> {
    // 获取当前分支的远程最新提交
    let fetch_output = Command::new("git")
        .current_dir(repo_path)
        .args(["fetch", "origin", branch])
        .output()?;

    if !fetch_output.status.success() {
        let error_msg = format!("Git fetch failed: {}", String::from_utf8_lossy(&fetch_output.stderr));
        error!("{}", error_msg);
        crate::webhook::send_webhook(webhook, "ERROR", &error_msg).await;
        return Err(anyhow::anyhow!(error_msg));
    }

    // 获取本地分支的当前提交
    let local_commit = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", "HEAD"])
        .output()?;

    // 获取远程分支的最新提交
    let remote_commit = Command::new("git")
        .current_dir(repo_path)
        .args(["rev-parse", &format!("origin/{}", branch)])
        .output()?;

    if !local_commit.status.success() || !remote_commit.status.success() {
        let error_msg = "Failed to get commit hashes".to_string();
        error!("{}", error_msg);
        crate::webhook::send_webhook(webhook, "ERROR", &error_msg).await;
        return Err(anyhow::anyhow!(error_msg));
    }

    let local_hash = String::from_utf8_lossy(&local_commit.stdout).trim().to_string();
    let remote_hash = String::from_utf8_lossy(&remote_commit.stdout).trim().to_string();

    if local_hash != remote_hash {
        // 有更新，执行 pull
        let pull_output = Command::new("git")
            .current_dir(repo_path)
            .args(["pull", "origin", branch])
            .output()?;

        if !pull_output.status.success() {
            let error_msg = format!("Git pull failed: {}", String::from_utf8_lossy(&pull_output.stderr));
            error!("{}", error_msg);
            crate::webhook::send_webhook(webhook, "ERROR", &error_msg).await;
            return Err(anyhow::anyhow!(error_msg));
        }

        let success_msg = format!("Successfully pulled new changes from {}", branch);
        info!("{}", success_msg);
        crate::webhook::send_webhook(webhook, "SUCCESS", &success_msg).await;
        Ok(true)
    } else {
        Ok(false)
    }
} 