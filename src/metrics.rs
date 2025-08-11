use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use tracing::info;

#[derive(Debug, Deserialize)]
struct PrometheusResponse {
    data: PrometheusData,
}

#[derive(Debug, Deserialize)]
struct PrometheusData {
    result: Vec<PrometheusResult>,
}

#[derive(Debug, Deserialize)]
struct PrometheusResult {
    metric: HashMap<String, String>,
    value: (f64, String),
}

pub async fn fetch_cpu_usage(
    prometheus_url: &str,
    namespace: &str,
) -> Result<HashMap<String, f64>> {
    info!("fetching cpu usage");
    let query = format!(
        "rate(container_cpu_usage_seconds_total{{namespace=\"{}\"}}[2m])",
        namespace
    );

    let url = format!(
        "{}/api/v1/query?query={}",
        prometheus_url,
        urlencoding::encode(&query)
    );

    let client = Client::new();
    let res = client
        .get(&url)
        .send()
        .await?
        .json::<PrometheusResponse>()
        .await?;

    let mut usage = HashMap::new();

    for item in res.data.result {
        if let Some(pod) = item.metric.get("pod") {
            let cpu = item.value.1.parse::<f64>().unwrap_or(0.0);
            usage.insert(pod.clone(), cpu);
        }
    }

    Ok(usage)
}
