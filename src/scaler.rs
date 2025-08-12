use std::sync::Arc;
use anyhow::Result;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{Api, Client};
use tracing::info;

use crate::observability::Metrics;

pub async fn scale_deployment_if_needed(
    client: Client,
    namespace: &str,
    deployment_name: &str,
    current_cpu: f64,
    scale_up_threshold: f64,
    scale_down_threshold: f64,
    metric: Arc<Metrics>
) -> Result<()> {
    let deployments: Api<Deployment> = Api::namespaced(client.clone(), namespace);

    let mut deployment = deployments.get(deployment_name).await?;

    let replicas = deployment
        .spec
        .as_ref()
        .and_then(|s| s.replicas)
        .unwrap_or(1);

    if current_cpu > scale_up_threshold {
        metric.scale_up_events_total.inc();

        let new_replicas = replicas + 1;
        info!(
            "🔼 Scaling up {} to {} replicas (CPU: {:.2})",
            deployment_name, new_replicas, current_cpu
        );
        deployment.spec.as_mut().unwrap().replicas = Some(new_replicas);
        deployments
            .replace(deployment_name, &Default::default(), &deployment)
            .await?;
    } else if current_cpu < scale_down_threshold && replicas > 1 {
        metric.scale_down_events_total.inc();

        let new_replicas = replicas - 1;
        info!(
            "🔽 Scaling down {} to {} replicas (CPU: {:.2})",
            deployment_name, new_replicas, current_cpu
        );
        deployment.spec.as_mut().unwrap().replicas = Some(new_replicas);
        deployments
            .replace(deployment_name, &Default::default(), &deployment)
            .await?;
    } else {
        info!(
            "🟢 No scaling needed for {} (CPU: {:.2})",
            deployment_name, current_cpu
        );
    }

    Ok(())
}
