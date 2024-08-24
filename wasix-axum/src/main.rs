use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Building our application with a single Route
    let app = Router::new().route("/", get(handler));

    // Run the server with hyper on http://127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀🚀listening on {}  🚀🚀", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, Axum ❤️ WASIX!"
}
