use evo_agent_sdk::{AgentRunner, kernel_handlers::EvaluationHandler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    AgentRunner::run(EvaluationHandler).await
}
