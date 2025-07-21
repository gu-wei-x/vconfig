mod config;
use config::index::IndexConfig;
use rocket::{get, routes};

#[get("/")]
async fn index(index_config: IndexConfig) -> String {
    index_config.welcome_msg
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket: rocket::Rocket<rocket::Ignite> =
        rocket::build().mount("/", routes![index]).launch().await?;
    Ok(())
}
