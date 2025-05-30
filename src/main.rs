mod config;
mod webhook;
mod builder;
mod git;

use anyhow::Result;
use clap::Parser;
use log::{info, error};
use std::time::Duration;
use tokio::time;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let args = Args::parse();
    let config = config::load_config(args.config.as_deref())?;
    
    info!("Starting autobuild with config: {:?}", config);
    
    loop {
        match git::check_and_pull(
            config.repository.to_str().unwrap(),
            &config.branch,
        ) {
            Ok(true) => {
                // 有更新，执行构建和发布
                if let Err(e) = builder::execute_command(&config.build, &config.webhook).await {
                    error!("Build failed: {}", e);
                    continue;
                }
                
                if let Err(e) = builder::execute_command(&config.publish, &config.webhook).await {
                    error!("Publish failed: {}", e);
                }
            }
            Ok(false) => {
                info!("No updates found");
            }
            Err(e) => {
                error!("Error checking for updates: {}", e);
            }
        }
        
        time::sleep(Duration::from_secs(config.interval)).await;
    }
}
