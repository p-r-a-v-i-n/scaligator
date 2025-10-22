use actix_web::{App, test, web};
use scaligator::alerts::handle_alerts;
use scaligator::observability::Metrics;
use std::sync::Arc;

#[actix_web::test]
async fn test_handle_alerts_endpoint() {
    let metrics = Arc::new(Metrics::new());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(metrics.clone()))
            .service(handle_alerts),
    )
    .await;

    let payload = serde_json::json!({
        "status": "firing",
        "alerts": [{
            "status": "firing",
            "labels": {
                "alertname": "HighCPUUsage",
                "namespace": "default",
                "pod": "demo-pod-123"
            },
            "annotations": {
                "cpu": "0.8"
            }
        }]
    });

    let req = test::TestRequest::post()
        .uri("/alerts")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
