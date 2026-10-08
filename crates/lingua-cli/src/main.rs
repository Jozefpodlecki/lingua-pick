extern crate alloc;

mod ai_call;
mod evaluator;
mod exercise;
mod generator;
mod logging;
mod migration;
mod runner;
mod seed;
mod simulator;
mod conversation;
mod store;
mod tools;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = runner::RunnerConfig::local()?;
    let run_id = uuid::Uuid::now_v7();
    logging::init(&config.log_directory, run_id)?;
    let result = async {
        let runner = runner::Runner::open(config)?;
        runner.run(run_id).await
    }
    .await;
    if let Err(error) = result {
        tracing::error!(%run_id, error = %error, "Run failed");
        return Err(error.into());
    }
    tracing::info!(%run_id, "Run completed");
    Ok(())
}
