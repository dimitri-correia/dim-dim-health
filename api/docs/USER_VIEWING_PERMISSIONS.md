# User Viewing Permissions System

## Overview

This system provides a reusable authorization framework that ensures users can only view data for:
1. **Their own data** (always allowed)
2. **Other users' data they have permission to watch** (via the `user_watch_permissions` table)

## Architecture

The system consists of three main components:

### 1. UserViewAuthorization Service (`api/src/auth/user_view_authorization.rs`)

The core authorization service that checks viewing permissions.

**Key Methods:**
- `can_view_user_data(requesting_user_id, target_user_id)` - Returns true if viewing is allowed
- `get_viewable_user_ids(requesting_user_id)` - Returns all user IDs the requesting user can view
- `verify_view_permission(requesting_user_id, target_user_id)` - Verifies permission or returns error

**Example Usage:**
```rust
let auth = app_state.services.authorization;
let can_view = auth.can_view_user_data(&requesting_user_id, &target_user_id).await?;
```

### 2. ViewUserData Extractor (`api/src/auth/resource_authorization.rs`)

An Axum extractor for route-level authorization (future implementation).

**Planned Usage:**
```rust
async fn get_user_weight(
    ViewUserData(target_user_id): ViewUserData,
    RequireAuth(requesting_user): RequireAuth,
) -> Result<Json<WeightResponse>, StatusCode> {
    // target_user_id is guaranteed to be viewable by requesting_user
}
```

### 3. Repository Helpers (future implementation)

Helper methods in repositories to filter queries by viewing permissions.

## How to Use in New Endpoints

### Option 1: Manual Authorization Check (Recommended for now)

Use the authorization service directly in your handlers:

```rust
use crate::auth::middleware::RequireAuth;
use crate::axummain::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

async fn get_user_data(
    State(app_state): State<AppState>,
    RequireAuth(requesting_user): RequireAuth,
    Path(target_user_id): Path<Uuid>,
) -> Result<Json<DataResponse>, StatusCode> {
    // Check if user has permission to view this data
    let can_view = app_state
        .services
        .authorization
        .can_view_user_data(&requesting_user.id, &target_user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if !can_view {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Fetch and return data...
    Ok(Json(data))
}
```

### Option 2: Get All Viewable IDs (For list endpoints)

When fetching lists of data, get all viewable user IDs and filter:

```rust
async fn list_user_weights(
    State(app_state): State<AppState>,
    RequireAuth(requesting_user): RequireAuth,
) -> Result<Json<Vec<WeightResponse>>, StatusCode> {
    // Get all user IDs this user can view
    let viewable_ids = app_state
        .services
        .authorization
        .get_viewable_user_ids(&requesting_user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Fetch weights for all viewable users
    let weights = weight_repository
        .find_by_user_ids(&viewable_ids)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(weights))
}
```

## Database Schema

The `user_watch_permissions` table structure:

```sql
CREATE TABLE user_watch_permissions (
    user_watched_id UUID NOT NULL,
    user_watching_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_watched_id, user_watching_id),
    FOREIGN KEY (user_watched_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (user_watching_id) REFERENCES users(id) ON DELETE CASCADE
);
```

**Relationship:**
- `user_watched_id`: The user whose data is being shared
- `user_watching_id`: The user who is allowed to view the data

## Testing

The core authorization logic is tested in `api/src/auth/user_view_authorization.rs` with the following test cases:

1. ✅ Users can view their own data
2. ✅ Users can view data when permission exists
3. ✅ Users cannot view data without permission
4. ✅ Verify permission success
5. ✅ Verify permission forbidden

## API Endpoints

### View Own Weights
- `GET /api/user/weights` - Get all your weights
- `GET /api/user/weights/last` - Get your last weight
- `GET /api/user/weights/infos` - Get your weight statistics

### View Another User's Weights (requires permission)
- `GET /api/users/{user_id}/weights` - Get all weights for a user you can watch
- `GET /api/users/{user_id}/weights/last` - Get last weight for a user you can watch
- `GET /api/users/{user_id}/weights/infos` - Get weight statistics for a user you can watch

### Manage Watch Permissions
- `GET /api/watch-permissions/watchers` - Get list of users watching you
- `GET /api/watch-permissions/watching` - Get list of users you are watching
- `POST /api/watch-permissions/grant` - Grant watch permission to another user
- `POST /api/watch-permissions/revoke` - Revoke watch permission from another user

## Future Enhancements

1. **ViewUserData Extractor**: Complete implementation with path parameter extraction
2. **Repository Helpers**: Add `find_by_user_ids()` methods to all user-data repositories
3. **Granular Permissions**: Extend to support different permission levels (read-only, read-write)
4. **Permission Management Endpoints**: Add API endpoints to create/delete watch permissions
5. **Caching**: Cache permission checks for better performance

## Security Considerations

1. **Always authenticate first**: Use `RequireAuth` before checking viewing permissions
2. **Never trust user IDs from client**: Always use the authenticated user's ID
3. **Check permissions for every request**: Don't cache permissions indefinitely
4. **Use database transactions**: When creating/deleting permissions with related data

## Support

For questions or issues with the viewing permissions system, refer to:
- Authorization service implementation: `api/src/auth/user_view_authorization.rs`
- Service integration: `api/src/services/mod.rs`
- Test examples: `api/src/auth/user_view_authorization.rs` (tests module)
