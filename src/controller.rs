use k8s_openapi::api::apps::v1::Deployment;
use kube::{Api, Client, ResourceExt};
use kube_runtime::watcher::{Config as WatcherConfig, Event, watcher};
use tokio::time::sleep;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

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
    namespace: Vec<String>
) -> anyhow::Result<()> {
    /*
    info!("👀 Watching deployments in {:?}", namespace);

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
*/

    info!("🚀 Controller starting reconciliation loop. Checking every {:?}.", config.reconcile_interval);

    loop {
        for ns in &namespace {
            info!("👀 Reconciling namespace: {:?}", ns);
            let deployments: Api<Deployment> = Api::namespaced(client.clone(), ns);

            match deployments.list(&Default::default()).await {
                Ok(deployment_list) => {
                    let usage_map = fetch_cpu_usage(&config.prometheus_url, ns).await?;
                    for deploy in deployment_list {
                        let deploy_name = deploy.name_any();

                        let mut total_cpu = 0.0;
                        let mut pod_count = 0;

                        for (pod, cpu) in &usage_map {
                            if extract_deployment_name(pod) == deploy_name {
                                total_cpu += cpu;
                                pod_count += 1;
                            }
                        }

                        if pod_count > 0 {
                            let avg_cpu = total_cpu / pod_count as f64;
                            scale_deployment_if_needed(
                                client.clone(),
                                ns,
                                &deploy_name,
                                avg_cpu,
                                config.scale_up_cpu_threshold,
                                config.scale_down_cpu_threshold,
                                metric.clone(),
                            )
                            .await?;
                        }
                    }
                },
                Err(e) => error!("Failed to list deployments in namespace {:?}, Error: {:?}", ns, e)
            }
        }
        info!("✅ Reconciliation complete. Sleeping for {:?}.", config.reconcile_interval);
        sleep(Duration::from_secs(config.reconcile_interval)).await;
    }
}





