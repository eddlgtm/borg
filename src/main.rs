mod types;
mod orchestrator;

use clap::{Arg, Command};
use orchestrator::Orchestrator;
// Removed redundant import
use types::*;

#[tokio::main]
async fn main() -> BorgResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let matches = Command::new("borg-coordinator")
        .version("0.1.0")
        .author("Claude AI")
        .about("AI Collaborative Development Orchestrator")
        .arg(
            Arg::new("redis-url")
                .long("redis-url")
                .value_name("URL")
                .help("Redis connection URL")
                .default_value("redis://localhost:6379"),
        )
        .arg(
            Arg::new("claude-code-path")
                .long("claude-code-path")
                .value_name("PATH")
                .help("Path to Claude Code executable")
                .default_value("claude"),
        )
        .get_matches();

    let config = OrchestratorConfig {
        redis_url: matches.get_one::<String>("redis-url").unwrap().clone(),
        claude_code_path: matches.get_one::<String>("claude-code-path").unwrap().clone(),
        ..Default::default()
    };

    let (orchestrator, _event_receiver) = Orchestrator::new(config).await?;
    orchestrator.initialize().await?;

    tracing::info!("Borg Coordinator started successfully");
    tracing::info!("Use Web UI: cargo run --bin web-ui, then open http://localhost:8080");

    // Start task processing loop
    let orch_clone = orchestrator.clone();
    tokio::spawn(async move {
        orch_clone.start_task_processing().await;
    });

    // Keep the process running
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
    loop {
        interval.tick().await;
        tracing::debug!("Borg Coordinator heartbeat");
    }
}