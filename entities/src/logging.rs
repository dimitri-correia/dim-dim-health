use serde::Deserialize;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

/// Configuration for logging setup
#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    /// Environment filter string (e.g., "info,my_crate=debug")
    pub env_filter: String,
    /// Output format: "pretty" for human-readable, "json" for structured logs
    #[serde(default = "default_format")]
    pub format: LogFormat,
    /// Whether to show file locations in logs
    #[serde(default = "default_true")]
    pub show_file: bool,
    /// Whether to show line numbers in logs
    #[serde(default = "default_true")]
    pub show_line_number: bool,
    /// Whether to show thread IDs in logs
    #[serde(default)]
    pub show_thread_ids: bool,
    /// Whether to show thread names in logs
    #[serde(default)]
    pub show_thread_names: bool,
    /// Whether to show span information
    #[serde(default)]
    pub show_span_events: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Pretty,
    Json,
    Compact,
}

fn default_format() -> LogFormat {
    LogFormat::Pretty
}

fn default_true() -> bool {
    true
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            env_filter: "info".to_string(),
            format: LogFormat::Pretty,
            show_file: true,
            show_line_number: true,
            show_thread_ids: false,
            show_thread_names: false,
            show_span_events: false,
        }
    }
}

impl LoggingConfig {
    /// Initialize the global tracing subscriber with this configuration
    pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        let env_filter = EnvFilter::try_new(&self.env_filter)
            .or_else(|_| EnvFilter::try_new("info"))?;

        let subscriber = tracing_subscriber::registry().with(env_filter);

        match self.format {
            LogFormat::Pretty => {
                let fmt_layer = fmt::layer()
                    .with_file(self.show_file)
                    .with_line_number(self.show_line_number)
                    .with_thread_ids(self.show_thread_ids)
                    .with_thread_names(self.show_thread_names)
                    .with_target(true)
                    .with_span_events(if self.show_span_events {
                        FmtSpan::NEW | FmtSpan::CLOSE
                    } else {
                        FmtSpan::NONE
                    })
                    .pretty();
                subscriber.with(fmt_layer).init();
            }
            LogFormat::Json => {
                let fmt_layer = fmt::layer()
                    .json()
                    .with_file(self.show_file)
                    .with_line_number(self.show_line_number)
                    .with_thread_ids(self.show_thread_ids)
                    .with_thread_names(self.show_thread_names)
                    .with_target(true)
                    .with_current_span(true)
                    .with_span_list(true);
                subscriber.with(fmt_layer).init();
            }
            LogFormat::Compact => {
                let fmt_layer = fmt::layer()
                    .compact()
                    .with_file(self.show_file)
                    .with_line_number(self.show_line_number)
                    .with_thread_ids(self.show_thread_ids)
                    .with_thread_names(self.show_thread_names)
                    .with_target(true)
                    .with_span_events(if self.show_span_events {
                        FmtSpan::NEW | FmtSpan::CLOSE
                    } else {
                        FmtSpan::NONE
                    });
                subscriber.with(fmt_layer).init();
            }
        }

        Ok(())
    }

    /// Create a simple logging config from an env_filter string
    pub fn from_env_filter(env_filter: &str) -> Self {
        Self {
            env_filter: env_filter.to_string(),
            ..Default::default()
        }
    }

    /// Builder pattern to configure format
    pub fn with_format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    /// Builder pattern to configure file display
    pub fn with_file(mut self, show: bool) -> Self {
        self.show_file = show;
        self
    }

    /// Builder pattern to configure line number display
    pub fn with_line_number(mut self, show: bool) -> Self {
        self.show_line_number = show;
        self
    }

    /// Builder pattern to configure thread ID display
    pub fn with_thread_ids(mut self, show: bool) -> Self {
        self.show_thread_ids = show;
        self
    }

    /// Builder pattern to configure thread name display
    pub fn with_thread_names(mut self, show: bool) -> Self {
        self.show_thread_names = show;
        self
    }

    /// Builder pattern to configure span events display
    pub fn with_span_events(mut self, show: bool) -> Self {
        self.show_span_events = show;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LoggingConfig::default();
        assert_eq!(config.env_filter, "info");
        assert_eq!(config.format, LogFormat::Pretty);
        assert!(config.show_file);
        assert!(config.show_line_number);
        assert!(!config.show_thread_ids);
    }

    #[test]
    fn test_from_env_filter() {
        let config = LoggingConfig::from_env_filter("debug");
        assert_eq!(config.env_filter, "debug");
        assert_eq!(config.format, LogFormat::Pretty);
    }

    #[test]
    fn test_builder_pattern() {
        let config = LoggingConfig::default()
            .with_format(LogFormat::Json)
            .with_thread_ids(true)
            .with_span_events(true);
        
        assert_eq!(config.format, LogFormat::Json);
        assert!(config.show_thread_ids);
        assert!(config.show_span_events);
    }
}
