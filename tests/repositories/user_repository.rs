#[tokio::test]
async fn test_create_user() {
    let username = "testrepocreateuser";
    let email = format!("{username}@test.fr");
    let password = "securepassword";

    let server = get_test_server().await;
}
