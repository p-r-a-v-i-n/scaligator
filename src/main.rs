mod alerts;
mod config;
mod controller;
mod metrics;
mod observability;
mod scaler;
mod cli;

use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    web::{self, Data},
};
use anyhow::Context;
use clap::Parser;
use config::AppConfig;
use controller::run_controller;
use kube::{Client, Config as KubeConfig};
use std::{path::PathBuf, sync::Arc};
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt};

use crate::{cli::Args, observability::Metrics};

#[get("/health")]
async fn health(metrics: web::Data<Arc<Metrics>>) -> impl Responder {
    metrics.http_requests_total.inc();

    HttpResponse::Ok().body("OK")
}

#[get("/ready")]
async fn ready(metrics: web::Data<Arc<Metrics>>) -> impl Responder {
    metrics.http_requests_total.inc();

    HttpResponse::Ok().body("READY")
}

#[get("/metrics")]
async fn metrics_handler(metrics: web::Data<Arc<Metrics>>) -> impl Responder {
    metrics.http_requests_total.inc();

    metrics.render()
}

fn init_logger() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_line_number(true)
        .with_target(true)
        .compact()
        .init();
}

#[tokio::main]
async fn main() {
    println!("🚀 Starting scaligator...");
    init_logger();
    info!("Logger initialized");

    if let Err(e) = run().await {
        error!("❌ Scaligator failed: {:#}", e);
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    // dotenvy::dotenv().context("Something went wrong during loading dotenv")?;

    let args = Args::parse();

    info!("🔧 Loading application config...");
    let app_config = AppConfig::configure(args.config.map(PathBuf::from)).context("Failed to load config")?;
    let namespaces: Vec<String> = app_config
        .watch_namespaces
        .split(",")
        .map(String::from)
        .collect();

    info!("✅ Config loaded: {:?}", app_config);

    info!("🔧 Inferring Kubernetes config...");
    let kube_config = KubeConfig::infer()
        .await
        .context("Failed to infer kube config")?;
    let kube_client = Client::try_from(kube_config).context("Failed to create kube client")?;
    info!("✅ Kubernetes client initialized");

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{port}");

    let observe_metrics = Arc::new(Metrics::new());
    let observe_metrics_app_data = Data::new(observe_metrics.clone());

    info!("🌐 Starting HTTP server on {}", bind_addr);
    let server = HttpServer::new(move || {
        App::new()
            .service(health)
            .service(ready)
            .service(alerts::handle_alerts)
            .app_data(observe_metrics_app_data.clone())
            .service(metrics_handler)
    })
    .bind(&bind_addr)?
    .run();

    tokio::select! {
        res = server => {
            res.context("HTTP server crashed")?;
        }
        res = run_controller(kube_client, app_config, observe_metrics,  namespaces) => {
            res.context("Controller crashed")?;
        }
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal. Exiting...");
        }
    }

    Ok(())
}
