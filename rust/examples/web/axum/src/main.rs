mod app_extensions;
mod handlers;
use axum::{Extension, Router, routing::get};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // create variants context.
    let variants_context =
        app_extensions::create_variants_context(&std::path::Path::new("./src/configs")).unwrap();
    let app = Router::new()
        .route("/", get(handlers::index))
        // add the variants context as an extension
        .layer(Extension(Arc::new(variants_context)));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
