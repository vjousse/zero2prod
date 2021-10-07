use zero2prod::run;

fn spawn_app() {
    let server = run("127.0.0.1:0").expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
