use anyhow::Result;
use prometheus::{proto::MetricFamily, IntCounter, Opts, Registry, TextEncoder};

pub struct Metrics {
    pub registry: Registry,
    pub scale_up_events_total: IntCounter,
    pub scale_down_events_total: IntCounter,
    pub http_requests_total: IntCounter
}

impl Metrics {
    fn new() -> Metrics {
        let registry = Registry::new();

        let scale_up_events_total_opts = Opts::new(
            "scale_up_events_total",
            "Total amount of scale up events"
        );

        let scale_down_events_total_opts = Opts::new(
            "scale_down_events_total",
            "Total amount of scale down events"
        ); 

        let http_requests_total_opts = Opts::new(
            "http_requests_total",
            "Total amount of http requests"
        );

        // NOTE: The with_opts() from [GenericCounter] only returns an error, If something is wrong
        // with the name of the Opts, Therefore a .expect() will suit it better
        let scale_up_events_total = IntCounter::with_opts(scale_up_events_total_opts).expect("Problem with the opts");
        let scale_down_events_total = IntCounter::with_opts(scale_down_events_total_opts).expect("Problem with the opts");
        let http_requests_total = IntCounter::with_opts(http_requests_total_opts).expect("Problem with the opts");

        // NOTE: Based on [RegistryCore], It checks if its unique, which will be better if solved before
        // prod, therefore a .expect is also used
        registry.register(Box::new(scale_up_events_total.clone())).expect("The collector has already been registered");
        registry.register(Box::new(scale_down_events_total.clone())).expect("The collector has already been registered");
        registry.register(Box::new(http_requests_total.clone())).expect("The collector has already been registered");

        Metrics { 
            registry,
            scale_up_events_total,
            scale_down_events_total,
            http_requests_total
        }
    }

    pub fn render(&self) -> Result<String> {
        let metric_family: Vec<MetricFamily> = self.registry.gather();

        let encoder = TextEncoder::new();
        let metric_buf =encoder.encode_to_string(&metric_family)?;

        Ok(metric_buf)
    }
}
