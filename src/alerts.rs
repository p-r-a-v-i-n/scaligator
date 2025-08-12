use crate::observability::Metrics;
use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Deserialize)]
struct AlertManagerWebhook {
    status: Option<String>,
    alerts: Vec<Alert>,
}

#[derive(Debug, Deserialize)]
struct Alert {
    status: Option<String>,
    labels: std::collections::HashMap<String, String>,
    annotations: std::collections::HashMap<String, String>,
}

#[post("/alerts")]
async fn handle_alerts(
    payload: web::Json<AlertManagerWebhook>,
    metrics: web::Data<Arc<Metrics>>,
) -> impl Responder {
    metrics.http_requests_total.inc();

    if let Some(global_status) = &payload.status {
        info!("📢 Global alert status: {}", global_status);
    }

    for alert in &payload.alerts {
        if let Some(alert_status) = &alert.status {
            info!("📢 Individual alert status: {}", alert_status);
        }

        if alert.labels.get("alertname") == Some(&"HighCPUUsage".to_string()) {
            let ns = alert
                .labels
                .get("namespace")
                .cloned()
                .unwrap_or_else(|| "default".to_string());
            let pod = alert.labels.get("pod").cloned().unwrap_or_default();
            let cpu_str = alert.annotations.get("cpu").cloned().unwrap_or_default();
            info!("🚨 Alert: pod={}, ns={}, cpu={}", pod, ns, cpu_str);

            // trigger scale logic if needed
        }
    }

    HttpResponse::Ok()
}
