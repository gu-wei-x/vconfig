extern crate variants as variantslib;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use variantslib::default::DefaultVariants;
use variantslib::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "variantslib::serde")]
//#[config("index")] // todo: add a macro to generate FromRequest impl automatically.
pub(crate) struct IndexConfig {
    pub(crate) welcome_msg: String,
}

// Required for async FromRequest implementations
#[rocket::async_trait]
impl<'r> FromRequest<'r> for IndexConfig {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let configs = request
            .rocket()
            .state::<crate::variants::config::VaraintsConfig>()
            .unwrap();
        match configs.get_file("index") {
            Some(path) => {
                let mut variants = DefaultVariants::default();
                configs.build_varaints(request, &mut variants);
                let config_result =
                    variantslib::de::from_file_with_variants::<IndexConfig, _, _>(path, &variants);
                match config_result {
                    Ok(config) => Outcome::Success(config),
                    _ => Outcome::Forward(rocket::http::Status { code: 500 }),
                }
            }
            _ => Outcome::Forward(rocket::http::Status { code: 500 }),
        }
    }
}
