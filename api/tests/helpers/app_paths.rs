pub struct TestAppPaths {
    // health check
    pub health_check: &'static str,
    // auth
    pub create_user: &'static str,
    pub create_guest_user: &'static str,
    pub current_user: &'static str,
    pub login_user: &'static str,
}

pub const APP_PATHS: TestAppPaths = TestAppPaths {
    health_check: "/health",
    create_user: "/api/users",
    create_guest_user: "/api/users/guest",
    current_user: "/api/user",
    login_user: "/api/users/login",
};
