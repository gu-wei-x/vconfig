mod app_state;
mod handlers;
use actix_web::{App, HttpServer, web};
use std::path::Path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new({
                let mut variants_context =
                    vconfig_actix_web::VariantsContext::new(&Path::new("src/configs")).unwrap();
                variants_context
                    .with_processor(app_state::variants_processors::BrowserVariants::default());
                variants_context
            }))
            .route("/", web::get().to(handlers::index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
