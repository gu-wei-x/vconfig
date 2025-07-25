mod configs;
mod handlers;
use actix_web::{App, HttpServer, web};
use std::path::Path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new({
                let mut variants_context =
                    variants_actix_web::VaraintsContext::new(&Path::new("src/configs")).unwrap();
                variants_context.with_processor(
                    configs::variants_processors::browser::BrowserVaraints::default(),
                );
                variants_context
            }))
            .route("/", web::get().to(handlers::index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
