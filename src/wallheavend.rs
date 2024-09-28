extern crate daemonize;

use axum::{
    extract::Path,
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use daemonize::Daemonize;
use serde_json::{json, Value};
use std::fs::File;
use std::process::Command;
use tokio::{runtime::Runtime, signal};
use tower_http::cors::CorsLayer;

fn main() {
    let output = Command::new("wallheaven").arg("-h").output();
    if !output.is_ok() {
        panic!("Wallheaven not found. http://github.com/davenicholson-xyz/wallheaven");
    }

    let app = Router::new()
        .route("/", get(ping))
        .route("/wp/:id", get(set_wallpaper))
        .route("/current", get(get_current))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:2388".parse::<HeaderValue>().unwrap())
                .allow_origin("https://wallhaven.cc".parse::<HeaderValue>().unwrap())
                .allow_methods(Method::GET),
        );

    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid")
        .stdout(stdout)
        .stderr(stderr);

    match daemonize.start() {
        Ok(_) => println!("Successfully daemonized"),
        Err(e) => eprintln!("Error: {}", e),
    }

    let runtime = Runtime::new().unwrap();

    runtime.block_on(async {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:2388").await.unwrap();
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap()
    })
}

async fn ping() -> Json<Value> {
    Json(json!({ "ping" : "pong"}))
}

async fn set_wallpaper(Path(id): Path<String>) -> impl IntoResponse {
    match Command::new("wallheaven").arg("--id").arg(&id).output() {
        Ok(output) => {
            if output.status.success() {
                return (StatusCode::OK, Json(json!({"wallpaper" : id })));
            } else {
                let msg = String::from_utf8_lossy(&output.stderr);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error" : msg.to_string().trim()})),
                );
            }
        }
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error" : e.to_string().trim()})),
            )
        }
    }
}

async fn get_current() -> Json<Value> {
    let output = Command::new("wallheaven")
        .arg("-u")
        .output()
        .expect("problemo");

    let current = String::from_utf8(output.stdout).expect("unknown");
    Json(json!({"current" : current.trim() }))
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
