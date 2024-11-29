use axum::{Router, routing::get, extract::Path, response::Html};
use std::fs;

async fn render_ssr(path: Path<String>) -> Html<String> {
    // Call your SolidJS SSR function (via `node`, or embed `deno` with Rust)
    let html = ssr_html(&path).await; // Stub for SSR logic
    Html(html)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/*path", get(render_ssr));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ssr_html(path: &str) -> String {
    // Example: Use `tokio::process::Command` to call `node` or `deno` SSR
    format!("<html><body>Rendered path: {}</body></html>", path)
}
