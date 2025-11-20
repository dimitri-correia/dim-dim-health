/// Integration test to verify backward compatibility of logging configuration
use entities::{LogFormat, LoggingConfig};

#[test]
fn test_backward_compatibility_default() {
    // Test that default configuration works
    let config = LoggingConfig::default();
    assert_eq!(config.env_filter, "info");
    assert_eq!(config.format, LogFormat::Pretty);
}

#[test]
fn test_from_env_filter_compat() {
    // Test backward compatibility method
    let config = LoggingConfig::from_env_filter("debug");
    assert_eq!(config.env_filter, "debug");
    assert_eq!(config.format, LogFormat::Pretty); // Default format
}

#[test]
fn test_builder_pattern_all_options() {
    // Test full builder pattern
    let config = LoggingConfig::from_env_filter("trace")
        .with_format(LogFormat::Json)
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_span_events(true);
    
    assert_eq!(config.env_filter, "trace");
    assert_eq!(config.format, LogFormat::Json);
    assert!(!config.show_file);
    assert!(!config.show_line_number);
    assert!(config.show_thread_ids);
    assert!(config.show_thread_names);
    assert!(config.show_span_events);
}

#[test]
fn test_serde_deserialization() {
    // Test TOML deserialization
    let toml_str = r#"
        env_filter = "dimdim_health=debug"
        format = "json"
        show_file = true
        show_line_number = true
        show_thread_ids = false
        show_thread_names = false
        show_span_events = false
    "#;
    
    let config: LoggingConfig = toml::from_str(toml_str).expect("Failed to deserialize");
    assert_eq!(config.env_filter, "dimdim_health=debug");
    assert_eq!(config.format, LogFormat::Json);
    assert!(config.show_file);
    assert!(config.show_line_number);
}

#[test]
fn test_format_variants() {
    // Test all format variants
    let json_config = LoggingConfig::default().with_format(LogFormat::Json);
    assert_eq!(json_config.format, LogFormat::Json);
    
    let pretty_config = LoggingConfig::default().with_format(LogFormat::Pretty);
    assert_eq!(pretty_config.format, LogFormat::Pretty);
    
    let compact_config = LoggingConfig::default().with_format(LogFormat::Compact);
    assert_eq!(compact_config.format, LogFormat::Compact);
}

#[test]
fn test_partial_toml_config() {
    // Test that defaults work when only env_filter is provided
    let toml_str = r#"
        env_filter = "warn"
    "#;
    
    let config: LoggingConfig = toml::from_str(toml_str).expect("Failed to deserialize");
    assert_eq!(config.env_filter, "warn");
    assert_eq!(config.format, LogFormat::Pretty); // Should use default
    assert!(config.show_file); // Should use default
    assert!(config.show_line_number); // Should use default
}
