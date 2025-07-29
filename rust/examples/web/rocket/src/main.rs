mod fairing;
mod handlers;

use crate::fairing::VariantsConfigFairing;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket: rocket::Rocket<rocket::Ignite> = rocket::build()
        .mount("/", handlers::routes())
        .attach(VariantsConfigFairing::default())
        .launch()
        .await?;
    Ok(())
}
