# Testing Guide

## Overview

This document explains the testing strategy for the dim-dim-health project and how to write tests that don't interfere with each other.

## The Problem

Previously, all tests shared the same database and ran concurrently, causing several issues:

1. **Data conflicts**: Tests creating users with the same username/email would fail randomly
2. **Race conditions**: Concurrent tests reading/writing the same data could interfere
3. **Non-deterministic failures**: Tests would pass or fail depending on execution order
4. **Difficult debugging**: Hard to reproduce issues that only occur when tests run in parallel

## The Solution

We use **serial test execution** with the `serial_test` crate to ensure database tests run one at a time, providing isolation without requiring complex transaction management or separate databases per test.

### Benefits

- ✅ **Simple implementation**: Just add `#[serial]` attribute to tests
- ✅ **Reliable**: Tests always run in a predictable order
- ✅ **Easy to understand**: No complex setup or teardown needed
- ✅ **Works with existing code**: Minimal changes to test infrastructure
- ✅ **Fast enough**: For most projects, serial execution is acceptable

### Trade-offs

- ⚠️ **Slower than parallel**: Tests run sequentially rather than concurrently
- ⚠️ **Shared state**: Tests still share the same database between runs

## Writing Tests

### For Database Tests

All tests that interact with the database should use the `#[serial]` attribute:

```rust
use serial_test::serial;

#[tokio::test]
#[serial]  // <-- Add this to ensure sequential execution
async fn test_create_user() {
    let app_state = get_app_state().await;
    // ... test code
}
```

### Best Practices

1. **Use unique test data**: Even with serial execution, use unique identifiers to prevent conflicts between test runs:
   ```rust
   let username = format!("testuser_{}", uuid::Uuid::new_v4());
   ```

2. **Clean up after yourself (optional)**: If your test creates persistent data, consider cleaning it up:
   ```rust
   // Create test data
   let user = create_user(&app_state).await;
   
   // Run test assertions
   assert_eq!(user.username, "testuser");
   
   // Optional: Clean up
   delete_user(&app_state, user.id).await;
   ```

3. **Don't rely on test execution order**: Each test should be independent and not assume data from previous tests exists.

4. **Use descriptive test names**: Make it clear what each test is testing.

### For Non-Database Tests

Tests that don't touch the database can run in parallel and don't need the `#[serial]` attribute:

```rust
#[tokio::test]
async fn test_password_hashing() {
    let password = "secure_password";
    let hash = hash_password(password).unwrap();
    assert!(verify_password(password, &hash).unwrap());
}
```

## Running Tests

### Run all tests (serially where marked)
```bash
cargo test
```

### Run specific test
```bash
cargo test test_create_user
```

### Run tests with output
```bash
cargo test -- --nocapture
```

### Run tests with more verbosity
```bash
cargo test -- --nocapture --test-threads=1
```

## Test Structure

```
api/tests/
├── endpoints/           # API endpoint tests (marked with #[serial])
│   ├── auth.rs         # Authentication tests
│   └── server_health.rs # Health check tests
├── repositories/       # Database repository tests (marked with #[serial])
│   ├── user_repository.rs
│   └── email_verification_repository.rs
└── mod.rs
```

## Database Setup

The test database is automatically set up by `scripts/test-db/run_test_db.sh`:
- Uses Podman to run PostgreSQL in a container
- Resets the database before each test run
- Runs on port 5433 (separate from development database on 5432)

## Future Improvements

Possible enhancements for even better test isolation:

1. **Transaction-based rollback**: Wrap each test in a database transaction and rollback after
2. **Database per test**: Create a new schema for each test (resource intensive)
3. **Mock database**: Use SeaORM's MockDatabase for unit tests
4. **Parallel execution with better isolation**: Implement test-specific namespacing

## Troubleshooting

### Tests are slow
- This is expected with serial execution
- Consider splitting unit tests (no DB) from integration tests (with DB)
- Run only the tests you need during development

### Tests fail intermittently
- Ensure all database tests have `#[serial]` attribute
- Check if test data uses unique identifiers
- Verify the database is properly reset between runs

### Database connection errors
- Ensure the test database is running: `./scripts/test-db/run_test_db.sh`
- Check if port 5433 is available
- Verify Podman/Docker is installed and running

## Additional Resources

- [serial_test crate documentation](https://docs.rs/serial_test/)
- [SeaORM testing guide](https://www.sea-ql.org/SeaORM/docs/write-test/testing/)
- [Rust testing best practices](https://doc.rust-lang.org/book/ch11-00-testing.html)
