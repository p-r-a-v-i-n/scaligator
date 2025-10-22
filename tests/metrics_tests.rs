use scaligator::observability::Metrics;

#[test]
fn test_metrics_increments() {
    let m = Metrics::new();

    m.scale_up_events_total.inc();
    m.scale_down_events_total.inc();
    m.http_requests_total.inc();

    assert_eq!(m.scale_up_events_total.get(), 1);
    assert_eq!(m.scale_down_events_total.get(), 1);
    assert_eq!(m.http_requests_total.get(), 1);
}

#[test]
fn test_metrics_render_output_not_empty() {
    let m = Metrics::new();
    let output = m.render();
    assert!(!output.is_empty(), "Metrics output should not be empty");
}
