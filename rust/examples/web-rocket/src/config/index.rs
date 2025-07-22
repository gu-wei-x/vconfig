extern crate variants as variantslib;
use crate::variants::browser::BrowserVaraints;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use variantslib::default::DefaultVariants;
use variantslib::default::VariantsBuilder;
use variantslib::serde::Deserialize;
//use variantslib::traits::Variants;

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
        let variants = request.local_cache::<DefaultVariants, _>(|| {
            let mut varaints_builder = VariantsBuilder::<Request<'_>, DefaultVariants>::default();
            varaints_builder.with_processor(Box::new(BrowserVaraints::default()));

            let mut varaints = DefaultVariants::default();
            varaints_builder.process_variants(request, &mut varaints);
            varaints
        });

        let config_result = variantslib::de::from_file_with_variants::<
            IndexConfig,
            _,
            DefaultVariants,
        >("./src/config/index.toml", &variants);

        match config_result {
            Ok(config) => Outcome::Success(config),
            _ => Outcome::Forward(rocket::http::Status { code: 500 }),
        }
    }
}
