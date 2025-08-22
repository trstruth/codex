#![allow(clippy::unwrap_used)]

use codex_core::config::ConfigToml;

#[test]
fn config_example_azure_msi_parses() {
    // Ensure the example config is kept valid.
    let toml_str = include_str!("../../docs/examples/config.azure-msi.toml");
    let cfg: ConfigToml = toml::from_str(toml_str).expect("example config.toml should parse");

    // Basic sanity checks on presence of entries referenced by profiles.
    assert!(cfg.model_providers.contains_key("azure-msi-chat"));
    assert!(cfg.profiles.contains_key("azure-msi-chat"));

    // Optional Responses variant should also be present.
    assert!(cfg.model_providers.contains_key("azure-msi-responses"));
    assert!(cfg.profiles.contains_key("azure-msi-responses"));
}
