use serde::Deserialize;
// use config::{Config, Environment, File};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub prometheus_url: String,
    pub watch_namespaces: Vec<String>,
    pub scale_up_cpu_threshold: f64,
    pub scale_down_cpu_threshold: f64,
    #[allow(dead_code)]
    pub disable_dev_after: Option<String>,
    #[allow(dead_code)]
    pub enable_dev_after: Option<String>,
}


// impl AppConfig {
//     pub fn from_env() -> Result<Self, config::ConfigError> {
//         let builder = Config::builder()
//             .add_source(File::with_name("Config").required(false))
//             .add_source(Environment::with_prefix("SCALIGATOR").separator("_"));
//         builder.build()?.try_deserialize()
//     }
// }
// //prometheus-k8s.monitoring.svc.cluster.local

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        Ok(AppConfig {
            prometheus_url: "http://localhost:9090".to_string(),
            watch_namespaces: vec!["default".into(), "scaling".into(), "dev".into()],
            scale_up_cpu_threshold: 0.7,
            scale_down_cpu_threshold: 0.2,
            disable_dev_after: Some("20:00".to_string()),
            enable_dev_after: Some("08:00".to_string()),
        })
    }
}
