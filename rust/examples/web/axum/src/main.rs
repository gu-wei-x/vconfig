mod app_extensions;
mod handlers;
use axum::{Extension, Router, routing::get};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // create variants context.
    let vconfig_context =
        app_extensions::create_vconfig_context(&std::path::Path::new("./src/configs")).unwrap();
    let app = Router::new()
        .route("/", get(handlers::index))
        // add the variants context as an extension
        .layer(Extension(Arc::new(vconfig_context)));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
