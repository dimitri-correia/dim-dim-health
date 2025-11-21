# Worker Optimization Implementation Guide

This directory contains optimized versions of the worker implementation with the recommended improvements from `WORKER_ANALYSIS.md`.

## Files

1. `optimized_state.rs` - Worker state with SMTP connection pooling
2. `optimized_worker.rs` - Enhanced worker loop with metrics and graceful shutdown
3. `optimized_common_mail_jobs.rs` - Email sending with connection reuse
4. `production.toml` - Production configuration example

## How to Apply These Optimizations

### Phase 1: SMTP Connection Pooling (30 minutes)

Replace the contents of `worker/src/worker_main/state.rs` with `optimized_state.rs`, making these key changes:

```rust
// Add to WorkerState struct
pub mailer: SmtpTransport,  // Persistent connection pool

// In the new() method, initialize once:
let mailer = SmtpTransport::relay("smtp.gmail.com")?
    .credentials(gmail_creds)
    .pool_config(PoolConfig::new().max_size(10).min_idle(2))
    .build();
```

Update `worker/src/mail_jobs/common_mail_jobs.rs` to use the pooled connection:

```rust
// Instead of creating new mailer each time, use:
match worker_state.mailer.send(&email) {
    // ...
}
```

### Phase 2: Metrics and Monitoring (1 hour)

Update `worker/src/worker_main/worker.rs` with timing and statistics tracking:

```rust
use std::time::Instant;
use std::sync::atomic::{AtomicU64, Ordering};

static JOBS_PROCESSED: AtomicU64 = AtomicU64::new(0);
static JOBS_FAILED: AtomicU64 = AtomicU64::new(0);
```

### Phase 3: Graceful Shutdown (30 minutes)

Add shutdown signal handling to allow workers to complete in-flight jobs before exiting.

### Phase 4: Configuration Updates (5 minutes)

Update `config/dev.toml`:
```toml
number_workers = 10  # Increased from 5
```

Create `config/production.toml` for production settings.

## Testing

After applying optimizations:

```bash
# 1. Build
cargo build --release

# 2. Run worker
./target/release/dimdim-health_worker

# 3. Load test
for i in {1..100}; do
  redis-cli RPUSH jobs '{"task_type":"Email","data":{"email_type":"Registration","data":{"email":"test@example.com","username":"testuser","token":"abc123"}}}'
done

# 4. Monitor performance
redis-cli LLEN jobs  # Check queue depth
grep "Completed job" logs/worker.log  # Check job timing
```

## Expected Performance Improvements

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Email throughput | 15-25/min | 40-60/min | +150% |
| SMTP connection time | 200-500ms/email | 10-20ms/email | -95% |
| Worker idle time | 30% | 10% | Better utilization |
| Job latency (P95) | 5s | 2s | -60% |

## Rollback Plan

If you encounter issues:

1. Keep the old files as `.backup`
2. Test in dev environment first
3. Monitor error rates after deployment
4. Revert by: `git revert HEAD` or restore `.backup` files

## Next Steps

After implementing these optimizations:

1. Monitor for 1 week in production
2. Measure actual throughput and latency improvements
3. Consider scaling to multiple worker processes if needed
4. Implement dead letter queue for failed jobs
5. Add Prometheus metrics for detailed monitoring
