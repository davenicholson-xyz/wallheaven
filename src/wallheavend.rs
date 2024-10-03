#[cfg(target_family = "unix")]
extern crate daemonize;
#[cfg(target_family = "unix")]
use std::fs::File;

use axum::{
    extract::Path,
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};

#[cfg(target_family = "unix")]
use daemonize::Daemonize;

#[cfg(target_family = "windows")]
use {std::env, std::process::Stdio};

use serde_json::{json, Value};
use std::process::Command;
use tokio::{runtime::Runtime, signal};
use tower_http::cors::CorsLayer;

fn main() {
    #[cfg(target_family = "windows")]
    let args: Vec<String> = env::args().collect();
    #[cfg(target_family = "windows")]
    let is_daemon = args.contains(&"--daemon".to_string());

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

    #[cfg(target_family = "unix")]
    daemonize_unix();

    #[cfg(target_family = "windows")]
    if !is_daemon {
        daemonize_windows();
        return;
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

#[cfg(target_family = "unix")]
fn daemonize_unix() {
    let stdout = File::create("/tmp/wallheavend.out").unwrap();
    let stderr = File::create("/tmp/wallheavend.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/wallheavend.pid")
        .stdout(stdout)
        .stderr(stderr);

    match daemonize.start() {
        Ok(_) => println!("Successfully daemonized"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(target_family = "windows")]
fn daemonize_windows() {
    use std::os::windows::process::CommandExt;
    const DETACHED_PROCESS: u32 = 0x00000008;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let result = Command::new(std::env::current_exe().unwrap())
        .arg("--daemon")
        .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    match result {
        Ok(_) => println!("Wallheavend running in background"),
        Err(e) => eprintln!("Failed to daemonize application: {}", e),
    }

    std::process::exit(0);
}
