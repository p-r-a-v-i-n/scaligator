use config::{Config, Environment, File};
use serde::Deserialize;
// use config::{Config, Environment, File};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub prometheus_url: String,
    pub watch_namespaces: String,
    pub scale_up_cpu_threshold: f64,
    pub scale_down_cpu_threshold: f64,
    pub reconcile_interval: u64,
    #[allow(dead_code)]
    pub disable_dev_after: Option<String>,
    #[allow(dead_code)]
    pub enable_dev_after: Option<String>,
}

 impl AppConfig {
     pub fn from_env() -> Result<Self, config::ConfigError> {
         let builder = Config::builder()
             .set_default("prometheus_url", "http://localhost:9090".to_string())?
             .set_default("watch_namespaces", "default, scaling, dev".to_string())?
             .set_default("scale_up_cpu_threshold", 0.7)?
             .set_default("scale_down_cpu_threshold", 0.2)?
             .set_default("reconcile_interval", 30)?
             .add_source(File::with_name("Config").required(false))
             .add_source(Environment::with_prefix("SCALIGATOR").separator("_"));
         builder.build()?.try_deserialize()
     }
}
// //prometheus-k8s.monitoring.svc.cluster.local
