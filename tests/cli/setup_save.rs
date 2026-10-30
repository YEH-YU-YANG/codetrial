//! What a Setup submission writes into the config file.
//!
//! Split out of `tests/cli.rs`, which is still the test target: cargo
//! discovers only `tests/*.rs`, so this compiles as a module of that one
//! binary rather than relinking the crate for a file of its own.

use super::*;

/// Accept, validate, write: the mocks answer as working credentials would,
/// LiveKit's `ListRooms` with 200, Gemini's handshake with `setupComplete`.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn setup_page_accepts_credentials_and_writes_the_primary_config_file() {
    let dir = exe_temp_path("setup-submit");
    std::fs::create_dir_all(&dir).unwrap();
    let exe = binary_beside(&dir);

    let livekit_addr = spawn_livekit_room_service("key", "secret");

    let gemini_mock = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let gemini_addr = gemini_mock.local_addr().unwrap();
    tokio::spawn(async move {
        if let Ok((socket, _)) = gemini_mock.accept().await
            && let Ok(mut socket) = accept_async(socket).await
        {
            let _ = socket.next().await;
            let _ = socket
                .send(tokio_tungstenite::tungstenite::Message::Text(
                    r#"{"setupComplete":{}}"#.into(),
                ))
                .await;
        }
    });

    let (addr, _server) = spawn_cold_start(
        &exe,
        &dir,
        &[("CODETRIAL_GEMINI_LIVE_URL", format!("ws://{gemini_addr}"))],
    );

    let body = format!(
        r#"{{"livekitUrl":"http://{livekit_addr}","livekitApiKey":"key","livekitApiSecret":"secret","googleApiKey":"google"}}"#
    );
    let response = http_request(
        &addr,
        &format!(
            "POST /api/setup HTTP/1.1\r\nHost: {addr}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        ),
    );

    let written = std::fs::read_to_string(dir.join("config").join("codetrial.env.local"))
        .expect("codetrial.env.local should have been written");

    // The file holds two secrets, so the umask does not get to decide who reads
    // it. Windows has no mode to check and inherits the folder's ACL.
    #[cfg(unix)]
    let mode = {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(dir.join("config").join("codetrial.env.local"))
            .expect("the written config should be readable")
            .permissions()
            .mode()
            & 0o777
    };
    let _ = std::fs::remove_dir_all(&dir);

    assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
    #[cfg(unix)]
    assert_eq!(mode, 0o600, "{mode:o}");
    assert!(
        written.contains(&format!("LIVEKIT_URL=http://{livekit_addr}")),
        "{written}"
    );
    assert!(written.contains("LIVEKIT_API_KEY=key"), "{written}");
    assert!(written.contains("LIVEKIT_API_SECRET=secret"), "{written}");
    assert!(written.contains("GOOGLE_API_KEY=google"), "{written}");
}

/// `googleApiKey` is optional, same as an operator's config file: omitting it
/// writes an empty `GOOGLE_API_KEY` and skips the Gemini live check, so no
/// Gemini mock is needed here.
#[test]
fn setup_page_accepts_credentials_without_a_google_api_key() {
    let dir = exe_temp_path("setup-submit-no-google-key");
    std::fs::create_dir_all(&dir).unwrap();
    let exe = binary_beside(&dir);

    let livekit_addr = spawn_livekit_room_service("key", "secret");

    let (addr, _server) = spawn_cold_start(&exe, &dir, &[]);

    let body = format!(
        r#"{{"livekitUrl":"http://{livekit_addr}","livekitApiKey":"key","livekitApiSecret":"secret","googleApiKey":""}}"#
    );
    let response = http_request(
        &addr,
        &format!(
            "POST /api/setup HTTP/1.1\r\nHost: {addr}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        ),
    );

    let written = std::fs::read_to_string(dir.join("config").join("codetrial.env.local"))
        .expect("codetrial.env.local should have been written");
    let _ = std::fs::remove_dir_all(&dir);

    assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");

    // Absent, not empty. `read_config_file` keeps an empty value and
    // `load_values` lays the file over the environment, so the line would erase
    // a `GOOGLE_API_KEY` the operator exported.
    assert!(!written.contains("GOOGLE_API_KEY"), "{written}");
}
