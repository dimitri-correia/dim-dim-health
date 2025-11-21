# Partial Entity Models

This module contains partial entity models (also called DTOs or Data Transfer Objects) that are used to optimize database queries by fetching only the required fields instead of all columns.

## Overview

When querying the database, fetching all columns of an entity can be inefficient, especially when:
- Only a few fields are needed for a specific operation
- The entity has many columns
- The query is executed frequently

Partial models solve this problem by defining lightweight structures that contain only the required fields. SeaORM's `FromQueryResult` trait combined with `select_only()` and `into_model()` methods allow us to efficiently query only specific columns.

## Benefits

1. **Reduced Network Traffic**: Less data transferred between database and application
2. **Improved Query Performance**: Database can optimize queries when fewer columns are selected
3. **Better Memory Usage**: Smaller structures use less memory
4. **Clearer Intent**: Partial models clearly communicate which fields are needed for specific operations

## Available Partial Models

### Users (`users_partial.rs`)

#### `UserAuthModel`
Used for authentication operations. Contains only the fields needed for login validation.

**Fields:**
- `id: Uuid`
- `username: String`
- `email: String`
- `password_hash: String`
- `email_verified: bool`

**Use Case:** Login, password verification

#### `UserPublicModel`
Used for displaying user information publicly.

**Fields:**
- `id: Uuid`
- `username: String`
- `email: String`
- `email_verified: bool`

**Use Case:** User profiles, listings

#### `UserBasicModel`
Used for basic user identification.

**Fields:**
- `id: Uuid`
- `username: String`
- `email: String`

**Use Case:** Simple lookups, user references

#### `UserEmailVerificationModel`
Used for email verification operations.

**Fields:**
- `id: Uuid`
- `email: String`
- `email_verified: bool`

**Use Case:** Email verification checks

### Tokens (`token_partial.rs`)

#### `RefreshTokenValidationModel`
Used for validating refresh tokens.

**Fields:**
- `user_id: Uuid`
- `expires_at: DateTimeWithTimeZone`
- `used_at: Option<DateTimeWithTimeZone>`

**Use Case:** Token validation, refresh token flow

#### `EmailVerificationTokenValidationModel`
Used for validating email verification tokens.

**Fields:**
- `user_id: Uuid`
- `expires_at: DateTimeWithTimeZone`

**Use Case:** Email verification token validation

## Usage Examples

### Using Partial Models in Repositories

```rust
use entities::users_partial::UserAuthModel;
use sea_orm::{EntityTrait, QuerySelect, ColumnTrait, QueryFilter};

// Query only auth-related fields
pub async fn find_by_email_for_auth(
    &self,
    email: &str,
) -> Result<Option<UserAuthModel>, sea_orm::DbErr> {
    users::Entity::find()
        .filter(users::Column::Email.eq(email.to_owned()))
        .select_only()
        .column(users::Column::Id)
        .column(users::Column::Username)
        .column(users::Column::Email)
        .column(users::Column::PasswordHash)
        .column(users::Column::EmailVerified)
        .into_model::<UserAuthModel>()
        .one(&self.db)
        .await
}
```

### Comparison: Full Model vs Partial Model

**Full Model Query:**
```rust
// Fetches all 7 columns: id, username, email, password_hash, created_at, updated_at, email_verified
let user = users::Entity::find()
    .filter(users::Column::Email.eq(email))
    .one(&db)
    .await?;
```

**Partial Model Query:**
```rust
// Fetches only 5 columns: id, username, email, password_hash, email_verified
let user = users::Entity::find()
    .filter(users::Column::Email.eq(email))
    .select_only()
    .column(users::Column::Id)
    .column(users::Column::Username)
    .column(users::Column::Email)
    .column(users::Column::PasswordHash)
    .column(users::Column::EmailVerified)
    .into_model::<UserAuthModel>()
    .one(&db)
    .await?;
```

## Best Practices

1. **Use Partial Models for Read Operations**: Use partial models for queries where you know exactly which fields you need
2. **Keep Full Models for Write Operations**: Use full entity models when creating or updating entities
3. **Name Clearly**: Name partial models based on their use case (e.g., `UserAuthModel` for authentication)
4. **Include All Required Fields**: Ensure partial models include all fields needed for their specific use case
5. **Document Use Cases**: Always document when and why a partial model should be used

## When to Create New Partial Models

Create a new partial model when:
- You have a frequently executed query that only needs a subset of fields
- The entity has many columns and you consistently use only a few
- You want to clearly communicate which fields are needed for a specific operation
- Performance profiling shows that full entity queries are a bottleneck

## Migration Guide

If you have existing code that uses full models, you can gradually migrate to partial models:

1. Identify queries that would benefit from partial models
2. Create the partial model if it doesn't exist
3. Add a new repository method that uses the partial model
4. Update calling code to use the new method
5. Monitor performance improvements

## Testing

All partial model repository methods should be tested to ensure:
- The query returns the expected fields
- The query correctly filters data
- The query handles edge cases (non-existing records, etc.)

See `api/tests/repositories/user_repository.rs` for examples.
