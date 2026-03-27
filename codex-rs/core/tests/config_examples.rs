#![allow(clippy::unwrap_used)]

use codex_core::config::ConfigToml;

#[test]
fn config_example_azure_msi_parses() {
    let toml_str = include_str!("../../docs/examples/config.azure-msi.toml");
    let cfg: ConfigToml = toml::from_str(toml_str).expect("example config.toml should parse");

    assert!(cfg.model_providers.contains_key("azure-msi"));
    assert!(cfg.profiles.contains_key("azure-msi"));
}
