use crate::auth::user_view_authorization::UserViewAuthorization;
use crate::repositories::user_watch_permission_repository::UserWatchPermissionRepository;
use sea_orm::DatabaseConnection;

pub mod authorization;

#[derive(Clone)]
pub struct Services {
    pub authorization: UserViewAuthorization,
}

impl Services {
    pub fn new(db: DatabaseConnection) -> Self {
        let watch_permission_repo = UserWatchPermissionRepository::new(db);
        let authorization = UserViewAuthorization::new(watch_permission_repo);

        Self { authorization }
    }
}
