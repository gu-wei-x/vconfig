mod handlers;
use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(handlers::index)))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
