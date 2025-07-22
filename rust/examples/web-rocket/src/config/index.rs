extern crate variants as variantslib;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use variantslib::default::DefaultVariants;
use variantslib::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "variantslib::serde")]
pub(crate) struct IndexConfig {
    pub(crate) welcome_msg: String,
}

// Required for async FromRequest implementations
#[rocket::async_trait]
impl<'r> FromRequest<'r> for IndexConfig {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut variants_builder = crate::variants::builder::VariantsBuilder::default();
        variants_builder.config();
        let mut variants = DefaultVariants::default();
        variants_builder.build(request, &mut variants);

        let config_result = variantslib::de::from_file_with_variants::<IndexConfig, _, _>(
            "./src/config/index.toml",
            &variants,
        );

        match config_result {
            Ok(config) => Outcome::Success(config),
            _ => Outcome::Forward(rocket::http::Status { code: 500 }),
        }
    }
}
