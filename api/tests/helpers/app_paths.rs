pub struct TestAppPaths {
    // health check
    pub health_check: &'static str,
    // auth
    pub create_user: &'static str,
    pub create_guest_user: &'static str,
    pub current_user: &'static str,
    pub login_user: &'static str,
    pub forgot_password: &'static str,
    pub reset_password: &'static str,
    pub reset_password_page: &'static str,
    // user groups
    pub join_public_group: &'static str,
    pub leave_public_group: &'static str,
    pub get_user_groups: &'static str,
    pub get_public_group_members: &'static str,
}

pub const APP_PATHS: TestAppPaths = TestAppPaths {
    health_check: "/health",
    create_user: "/api/users",
    create_guest_user: "/api/users/guest",
    current_user: "/api/user",
    login_user: "/api/users/login",
    forgot_password: "/api/auth/forgot-password",
    reset_password: "/api/auth/reset-password",
    reset_password_page: "/api/auth/reset-password",
    join_public_group: "/api/user-groups/join-public",
    leave_public_group: "/api/user-groups/leave-public",
    get_user_groups: "/api/user-groups/myself",
    get_public_group_members: "/api/user-groups/public/members",
};
