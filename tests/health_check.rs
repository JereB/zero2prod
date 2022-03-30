use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // create running server and client
    let addr = spawn_app();
    let client = reqwest::Client::new();

    // execute request against server
    let response = client
        .get(addr + "/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    // check if request is always successful and body is empty
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to randowm port");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Could not bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
