mod config;
mod controller;
mod scaler;
mod metrics;

use config::AppConfig;
use controller::run_controller;
use kube::{Client, Config};
use tracing_subscriber::{fmt, EnvFilter};

fn init_logger() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stdout)
        .with_ansi(false)            
        .with_line_number(true)
        .with_target(true)
        .compact()
        .init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    init_logger();

    let app_config = AppConfig::from_env()?;
    tracing::info!("Loaded config: {:?}", app_config);

    let kube_config = Config::infer().await?;
    let kube_client = Client::try_from(kube_config)?;

    run_controller(kube_client, app_config).await?;

    Ok(())
}
