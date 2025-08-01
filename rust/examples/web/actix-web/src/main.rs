mod app_state;
mod handlers;
use actix_web::{App, HttpServer, web};
use std::path::Path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new({
                let mut vconfig_context =
                    vconfig_actix_web::VConfigContext::new(&Path::new("src/configs")).unwrap();
                vconfig_context
                    .with_processor(app_state::variants_processors::BrowserVariants::default());
                vconfig_context
            }))
            .route("/", web::get().to(handlers::index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
