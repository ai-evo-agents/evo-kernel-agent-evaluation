use evo_agent_sdk::{AgentRunner, kernel_handlers::EvaluationHandler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::args().any(|a| a == "--version" || a == "-V") {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    AgentRunner::run(EvaluationHandler).await
}
