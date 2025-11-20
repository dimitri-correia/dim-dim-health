# Logging System

This project uses a structured logging system based on the `tracing` crate, providing comprehensive logging capabilities for both the API server and worker processes.

## Features

- **Structured Logging**: All logs use the `tracing` crate for structured, contextual logging
- **Multiple Output Formats**: Support for pretty-printed, JSON, and compact log formats
- **Request Tracing**: Automatic request ID generation and correlation for API requests
- **Configurable Levels**: Fine-grained control over log levels per module
- **Performance Metrics**: Automatic latency tracking for HTTP requests
- **Backward Compatible**: Supports legacy `env_filter` configuration

## Configuration

Logging is configured in the `config/*.toml` files using the `[logging]` section:

```toml
[logging]
# Log level filter (using env_filter syntax)
env_filter = "dimdim_health=debug,tower_http=debug"

# Output format: "pretty", "json", or "compact"
format = "pretty"

# Include file names in logs
show_file = true

# Include line numbers in logs
show_line_number = true

# Show thread IDs (useful for debugging concurrency issues)
show_thread_ids = false

# Show thread names
show_thread_names = false

# Show span lifecycle events (NEW/CLOSE)
show_span_events = false
```

### Log Formats

#### Pretty Format (Development)
Human-readable format with color coding and easy-to-read structure. Best for development.

```bash
2025-11-20T10:15:30.123456Z  INFO request{method=POST uri=/api/users version=HTTP/1.1 request_id=a1b2c3d4-e5f6-7890-abcd-ef1234567890}: dimdim_health_api::handlers::auth: Received registration request for: john_doe [email: john@example.com]
```

#### JSON Format (Production)
Machine-readable structured logs suitable for log aggregation systems (ELK, Splunk, etc.).

```json
{
  "timestamp": "2025-11-20T10:15:30.123456Z",
  "level": "INFO",
  "fields": {
    "message": "Received registration request for: john_doe [email: john@example.com]"
  },
  "target": "dimdim_health_api::handlers::auth",
  "span": {
    "method": "POST",
    "uri": "/api/users",
    "request_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
  }
}
```

#### Compact Format
Condensed format with minimal whitespace, useful for high-volume logging.

## Usage in Code

### Basic Logging

Use the `tracing` macros throughout the codebase:

```rust
use tracing::{info, debug, warn, error};

// Information logging
info!("Server starting on port 3000");

// Debug details
debug!(user_id = %user.id, "Processing user request");

// Warnings
warn!(attempt_count = 3, "Multiple failed login attempts");

// Errors
error!(error = %err, "Failed to connect to database");
```

### Structured Fields

Add structured fields to your logs for better searchability:

```rust
use tracing::info;

info!(
    user_id = %user.id,
    email = %user.email,
    "User registered successfully"
);
```

### Spans for Request Context

Create spans to group related log entries:

```rust
use tracing::{info, info_span};

async fn process_order(order_id: Uuid) {
    let _span = info_span!("process_order", order_id = %order_id).entered();
    
    info!("Starting order processing");
    // All logs here will be associated with this order_id
    info!("Order validation complete");
    info!("Order processed successfully");
}
```

## Request Tracing

The API server automatically adds request tracing with the following information:

- **Request ID**: Unique UUID for each request
- **HTTP Method**: GET, POST, etc.
- **URI Path**: Request endpoint
- **Status Code**: Response status
- **Latency**: Request processing time in milliseconds

Example log entry:
```
INFO request{method=POST uri=/api/users/login request_id=...}: started processing request
INFO request{method=POST uri=/api/users/login request_id=...}: Received login request for email: user@example.com
INFO request{method=POST uri=/api/users/login request_id=...}: request completed successfully status=200 latency_ms=45
```

## Environment Filter Syntax

The `env_filter` field uses a flexible syntax to control log levels:

```toml
# Set global level to info
env_filter = "info"

# Set specific crate to debug
env_filter = "dimdim_health=debug"

# Multiple crates with different levels
env_filter = "dimdim_health=debug,tower_http=info,sqlx=warn"

# Enable all logs (verbose)
env_filter = "trace"
```

### Log Levels (from most to least verbose)
1. `trace` - Very detailed, typically not enabled
2. `debug` - Detailed information for debugging
3. `info` - General informational messages
4. `warn` - Warning messages
5. `error` - Error messages

## Migration from `log` crate

If you encounter code using the old `log` crate, convert it to `tracing`:

### Before (log):
```rust
use log::{info, error};

info!("User {} logged in", username);
error!("Database connection failed: {}", err);
```

### After (tracing):
```rust
use tracing::{info, error};

info!(username = %username, "User logged in");
error!(error = %err, "Database connection failed");
```

## Best Practices

1. **Use Appropriate Levels**:
   - `error!` for errors that need attention
   - `warn!` for concerning but non-critical issues
   - `info!` for important application events
   - `debug!` for detailed diagnostic information

2. **Add Context with Fields**:
   ```rust
   info!(user_id = %user_id, action = "login", "User action");
   ```

3. **Use Spans for Operations**:
   ```rust
   let _span = info_span!("database_query", query = "SELECT * FROM users").entered();
   ```

4. **Don't Log Sensitive Data**:
   - Avoid logging passwords, tokens, or personal information
   - Use `%` formatting for Display, `?` for Debug
   - Consider redacting sensitive fields

5. **Keep Messages Concise**:
   - Use fields for data, messages for context
   - Good: `info!(count = 5, "Items processed")`
   - Bad: `info!("Processed 5 items")`

## Performance Considerations

- Logging has minimal performance impact when using appropriate levels
- JSON format is slightly faster than pretty format
- Disable `show_span_events` in production for better performance
- Use `debug!` and `trace!` levels for verbose information that can be filtered out in production

## Production Recommendations

For production environments, use this configuration:

```toml
[logging]
env_filter = "dimdim_health=info,tower_http=warn"
format = "json"
show_file = true
show_line_number = true
show_thread_ids = false
show_thread_names = false
show_span_events = false
```

This provides:
- Structured JSON logs for log aggregation systems
- Appropriate log levels to reduce noise
- File and line information for debugging
- Minimal performance overhead
