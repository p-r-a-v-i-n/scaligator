use futures_util::{pin_mut, stream::TryStreamExt};
use k8s_openapi::api::apps::v1::Deployment;
use kube::{Api, Client, ResourceExt};
use kube_runtime::watcher::{Config as WatcherConfig, Event, watcher};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

use crate::config::AppConfig;
use crate::metrics::fetch_cpu_usage;
use crate::observability::Metrics;
use crate::scaler::scale_deployment_if_needed;

fn extract_deployment_name(pod_name: &str) -> String {
    pod_name
        .rsplitn(3, '-')
        .last()
        .unwrap_or(pod_name)
        .to_string()
}

pub async fn run_controller(
    client: Client,
    config: AppConfig,
    metric: Arc<Metrics>,
) -> anyhow::Result<()> {
    info!("👀 Watching deployments in {:?}", config.watch_namespaces);

    let deployments: Api<Deployment> = Api::all(client.clone());
    let watcher = watcher(deployments, WatcherConfig::default());
    pin_mut!(watcher);

    while let Some(event) = watcher.try_next().await? {
        match event {
            Event::Applied(deploy) => {
                let deploy_name = deploy.name_any();
                let ns = deploy.namespace().unwrap_or_else(|| "default".into());
                info!("🆕 Applied: {deploy_name} in ns {ns}");

                let usage_map: HashMap<String, f64> =
                    fetch_cpu_usage(&config.prometheus_url, &ns).await?;

                for (pod, cpu) in usage_map {
                    let deployment = extract_deployment_name(&pod);
                    if deployment == deploy_name {
                        scale_deployment_if_needed(
                            client.clone(),
                            &ns,
                            &deploy_name,
                            cpu,
                            config.scale_up_cpu_threshold,
                            config.scale_down_cpu_threshold,
                            metric.clone(),
                        )
                        .await?;
                    }
                }
            }
            Event::Deleted(deploy) => {
                info!(
                    "❌ Deleted: {} in ns {}",
                    deploy.name_any(),
                    deploy.namespace().unwrap_or_default()
                );
            }
            Event::Restarted(deploys) => {
                info!("🔄 Restarted watcher with {} deployments", deploys.len());
            }
        }
    }

    Ok(())
}
