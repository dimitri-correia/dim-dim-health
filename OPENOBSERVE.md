# OpenObserve Integration Guide

## Overview

DimDim Health now integrates with **OpenObserve**, an open-source observability platform that provides logs, metrics, and traces collection and visualization.

## Is OpenObserve Free to Use?

**Yes!** OpenObserve is completely free to use:

- ✅ **Open Source**: Apache 2.0 license
- ✅ **Self-hosted**: Run on your own infrastructure
- ✅ **No vendor lock-in**: Full control over your data
- ✅ **Cost-effective**: 140x lower storage costs compared to Elasticsearch
- ✅ **Single binary**: Easy to deploy and maintain
- ✅ **Built-in UI**: No additional tools needed for visualization

## Features

- **Unified Observability**: Logs, metrics, and traces in one platform
- **OpenTelemetry Native**: Full OTLP (OpenTelemetry Protocol) support
- **High Performance**: Built in Rust for memory safety and speed
- **Low Resource Usage**: Requires only 1/4th the hardware compared to Elasticsearch
- **S3-Native Storage**: Leverage cheap object storage with intelligent caching
- **Real-time Search**: Fast queries with SQL support
- **Multi-tenant**: Organizations and streams as first-class concepts

## Getting Started

### Prerequisites

- Docker and Docker Compose installed on your system

### 1. Start OpenObserve with Docker Compose

The easiest way to run OpenObserve is using the provided `docker-compose.yml`:

```bash
docker-compose up -d openobserve
```

This will start OpenObserve with the following default configuration:
- **UI/API Port**: 5080
- **Default Organization**: default
- **Admin Email**: admin@example.com
- **Admin Password**: Complexpass#123

### 2. Access the OpenObserve UI

Open your browser and navigate to:
```
http://localhost:5080
```

Login with the credentials:
- **Email**: admin@example.com
- **Password**: Complexpass#123

### 3. Configure DimDim Health to Use OpenObserve

Edit your configuration file (e.g., `config/dev.toml`) and uncomment the OpenObserve endpoint:

```toml
# OpenObserve configuration
openobserve_endpoint = "http://localhost:5080/api/default"
```

The format is: `http://<host>:<port>/api/<org_name>`

### 4. Start Your Applications

Start the API server and worker:

```bash
# Terminal 1: API Server
cargo run --bin dimdim-health_api

# Terminal 2: Worker
cargo run --bin dimdim-health_worker
```

### 5. View Logs in OpenObserve

1. Go to http://localhost:5080
2. Login with your credentials
3. Navigate to **Logs** in the sidebar
4. Select the stream:
   - `dimdim-health-api` for API logs
   - `dimdim-health-worker` for worker logs
5. You'll see all logs and traces from your applications

## Configuration Options

### Environment Variables for OpenObserve

You can customize OpenObserve by setting environment variables in `docker-compose.yml`:

```yaml
environment:
  ZO_ROOT_USER_EMAIL: "your-email@example.com"
  ZO_ROOT_USER_PASSWORD: "YourSecurePassword"
  ZO_DATA_DIR: "/data"
  # Optional: Configure S3 storage
  # ZO_S3_BUCKET_NAME: "your-bucket"
  # ZO_S3_REGION_NAME: "us-west-2"
```

### Application Configuration

In your `config/*.toml` files:

```toml
# Enable OpenObserve logging
openobserve_endpoint = "http://localhost:5080/api/default"

# Or disable by commenting out or setting to empty
# openobserve_endpoint = ""
```

### Log Levels

Control what gets logged using the `env_filter` setting:

```toml
env_filter = "dimdim_health=debug,tower_http=debug"
```

Levels: `trace`, `debug`, `info`, `warn`, `error`

## Advanced Usage

### Using OpenTelemetry Features

The integration uses OpenTelemetry, which provides:

1. **Distributed Tracing**: Track requests across services
2. **Structured Logging**: Logs are structured with metadata
3. **Correlation**: Trace IDs link logs and spans together

### Querying Logs

OpenObserve supports SQL queries for logs:

```sql
SELECT * FROM "dimdim-health-api" 
WHERE level = 'error' 
ORDER BY _timestamp DESC 
LIMIT 100
```

### Setting Up Alerts

1. Go to **Alerts** in the OpenObserve UI
2. Create a new alert rule
3. Set conditions (e.g., error count > 10 in 5 minutes)
4. Configure notifications (email, Slack, etc.)

### Dashboards

Create custom dashboards to visualize:
- Request rates
- Error rates
- Response times
- Custom metrics

## Production Deployment

### High Availability Setup

For production, consider:

1. **Persistent Storage**: Use S3, MinIO, or GCS
2. **Multiple Nodes**: Run OpenObserve in cluster mode
3. **Load Balancer**: Distribute traffic across nodes
4. **Backup**: Regular backups of OpenObserve data
5. **TLS/SSL**: Enable HTTPS for security

### Example Production Configuration

```yaml
openobserve:
  image: public.ecr.aws/zinclabs/openobserve:latest
  environment:
    ZO_ROOT_USER_EMAIL: "admin@yourcompany.com"
    ZO_ROOT_USER_PASSWORD: "${OPENOBSERVE_PASSWORD}"  # Use secrets
    ZO_S3_BUCKET_NAME: "your-logs-bucket"
    ZO_S3_REGION_NAME: "us-east-1"
    ZO_CLUSTER_ENABLED: "true"
  volumes:
    - /path/to/persistent/data:/data
```

## Troubleshooting

### Logs not appearing in OpenObserve

1. Check if OpenObserve is running:
   ```bash
   docker ps | grep openobserve
   ```

2. Verify the endpoint is correct in your config file

3. Check application logs for connection errors:
   ```bash
   cargo run --bin dimdim-health_api 2>&1 | grep -i openobserve
   ```

### Connection refused errors

- Ensure OpenObserve is running and healthy
- Check if the port 5080 is accessible
- Verify firewall rules

### High memory usage

- OpenObserve uses memory for caching
- Adjust cache size in configuration if needed
- Consider using S3 storage for large datasets

## Resources

- **OpenObserve GitHub**: https://github.com/openobserve/openobserve
- **Documentation**: https://openobserve.ai/docs/
- **Community**: https://github.com/openobserve/openobserve/discussions
- **OpenTelemetry**: https://opentelemetry.io/

## Comparison with Other Solutions

| Feature | OpenObserve | Elasticsearch | Datadog |
|---------|-------------|---------------|---------|
| **Cost** | Free (self-hosted) | Expensive | Very expensive |
| **Storage Cost** | 140x lower | Baseline | High |
| **Setup Complexity** | Single binary | Complex | SaaS |
| **Performance** | High | Good | High |
| **Resource Usage** | Low (1/4th) | High | N/A |
| **OpenTelemetry** | Native | Via plugin | Partial |
| **UI** | Built-in | Kibana (separate) | Built-in |

## License

OpenObserve is licensed under the Apache License 2.0, making it completely free for both personal and commercial use.
