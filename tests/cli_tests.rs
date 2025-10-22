use clap::Parser;
use scaligator::cli::Args;

#[test]
fn test_cli_with_config() {
    let args = Args::parse_from(&["scaligator", "--config", "config.yaml"]);
    assert_eq!(args.config.as_deref(), Some("config.yaml"));
}

#[test]
fn test_cli_without_config() {
    let args = Args::parse_from(&["scaligator"]);
    assert!(args.config.is_none());
}
