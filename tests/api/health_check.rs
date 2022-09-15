#[tokio::test]
async fn health_check_works() {
    let addr = spwan_app_at_port(8000);

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spwan_app_at_port(port: u16) -> String {
    let server = rustymail::run_app(port).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
