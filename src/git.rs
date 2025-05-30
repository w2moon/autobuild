use std::process::Command;
use anyhow::Result;
use log::{info, error};

pub fn check_and_pull(repo_path: &str, branch: &str) -> Result<bool> {
    // 获取当前分支的远程最新提交
    let fetch_output = Command::new("git")
        .current_dir(repo_path)
        .args(["fetch", "origin", branch])
        .output()?;

    if !fetch_output.status.success() {
        error!("Git fetch failed: {}", String::from_utf8_lossy(&fetch_output.stderr));
        return Err(anyhow::anyhow!("Git fetch failed"));
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
        return Err(anyhow::anyhow!("Failed to get commit hashes"));
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
            error!("Git pull failed: {}", String::from_utf8_lossy(&pull_output.stderr));
            return Err(anyhow::anyhow!("Git pull failed"));
        }

        info!("Successfully pulled new changes from {}", branch);
        Ok(true)
    } else {
        Ok(false)
    }
} 