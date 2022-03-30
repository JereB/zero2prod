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

#[tokio::test]
async fn subscribe_returns_200_with_valid_form_data() {
    // setup
    let addr = spawn_app();
    let client = reqwest::Client::new();

    // execute request
    let body = "name=Mr%20Robot&email=mr%40robots.com";

    let response = client
        .post(addr + "/subscriptions")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // asser invariant
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_with_incomlete_form_data() {
    //setup
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let payloads = vec![
        ("", "empty"),
        ("name=Mr%20Robot", "missing the email"),
        ("email=mr%40robots.com", "missing the name"),
    ];

    // for each invalid body
    for (payload, data_descrp) in payloads {
        //execute request
        let response = client
            .post(format!("{}/subscriptions", &addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(payload)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "Api did not return 400 status code when the form data was {}",
            data_descrp
        );
    }
}
