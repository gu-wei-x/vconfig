use actix_web::Responder;

use variants_actix_web::serde::Deserialize;
//use variants_actix_web::variants_config;

#[derive(Debug, Deserialize)]
#[serde(crate = "variants_actix_web::serde")]
//#[variants_config("index")]
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

pub(crate) async fn index(index_config: IndexConfig) -> impl Responder {
    index_config.welcome_msg
}

impl actix_web::FromRequest for IndexConfig {
    type Error = actix_web::error::InternalError<String>;
    type Future = std::pin::Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        request: &actix_web::HttpRequest,
        _: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let variants_context = request
            .app_data::<actix_web::web::Data<variants_actix_web::VaraintsContext>>()
            .unwrap();
        match variants_context.get_file("index") {
            Some(path) => {
                let mut variants = variants_actix_web::default::DefaultVariants::default();
                variants_context.build_varaints(request, &mut variants);
                let config_result =
                    variants_actix_web::de::from_file_with_variants::<IndexConfig, _, _>(
                        path, &variants,
                    );
                match config_result {
                    Ok(config) => Box::pin(async move { Ok(config) }),
                    _ => Box::pin(async move {
                        Err(actix_web::error::InternalError::new(
                            "Deserilize error".to_owned(),
                            actix_web::http::StatusCode::NOT_IMPLEMENTED,
                        ))
                    }),
                }
            }
            _ => Box::pin(async move {
                Err(actix_web::error::InternalError::new(
                    "Deserilize error".to_owned(),
                    actix_web::http::StatusCode::NOT_IMPLEMENTED,
                ))
            }),
        }
    }
}
