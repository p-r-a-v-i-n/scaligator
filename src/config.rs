use std::path::PathBuf;

use config::{Config, Environment, File};
use serde::Deserialize;
use tracing::warn;
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
    pub fn configure(path: Option<PathBuf>) -> Result<Self, config::ConfigError> {
        // Subtitute it if path isnt provided
        let config_path: PathBuf = match path {
            Some(s) => {
                if !s.exists() {
                    warn!(
                        "❔ Config file from argument doesn\'t exist, Using default config path, cli argument: {:#?}",
                        s
                    );
                    let t = PathBuf::from("./Config");
                    if !t.exists() {
                        warn!(
                            "❔ Default config file doesn\'t exist, using Environment and default config"
                        );
                    }
                    t
                } else {
                    s
                }
            }
            None => {
                let t = PathBuf::from("./Config");
                if !t.exists() {
                    warn!(
                        "❔ Default config file doesn\'t exist, using Environment and default config"
                    );
                }
                t
            }
        };

        let builder = Config::builder()
            .set_default("prometheus_url", "http://localhost:9090".to_string())?
            .set_default("watch_namespaces", "default, scaling, dev".to_string())?
            .set_default("scale_up_cpu_threshold", 0.7)?
            .set_default("scale_down_cpu_threshold", 0.2)?
            .set_default("reconcile_interval", 30)?
            .add_source(File::from(config_path).required(false))
            .add_source(Environment::with_prefix("SCALIGATOR").separator("_"));
        builder.build()?.try_deserialize()
    }
}
// //prometheus-k8s.monitoring.svc.cluster.local
