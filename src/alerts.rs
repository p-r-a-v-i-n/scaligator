use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;
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
async fn handle_alerts(payload: web::Json<AlertManagerWebhook>) -> impl Responder {
    for alert in &payload.alerts {
        if alert.labels.get("alertname") == Some(&"HighCPUUsage".to_string()) {
            let ns = alert
                .labels
                .get("namespace")
                .cloned()
                .unwrap_or_else(|| "default".to_string());
            let pod = alert.labels.get("pod").cloned().unwrap_or_default();
            let cpu_str = alert.annotations.get("cpu").cloned().unwrap_or_default();
            info!("🚨 Alert: pod={}, ns={}, cpu={}", pod, ns, cpu_str);

            // Here you can trigger your scale logic if needed.
        }
    }

    HttpResponse::Ok()
}
