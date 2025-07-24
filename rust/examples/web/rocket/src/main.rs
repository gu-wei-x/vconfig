mod fairing;
mod handlers;

use crate::fairing::VaraintsConfigFairing;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket: rocket::Rocket<rocket::Ignite> = rocket::build()
        .mount("/", handlers::routes())
        .attach(VaraintsConfigFairing::default())
        .launch()
        .await?;
    Ok(())
}
