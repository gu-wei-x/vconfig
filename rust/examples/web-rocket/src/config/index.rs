use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use variants::default;
use variants::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "variants::serde")]
pub(crate) struct IndexConfig {
    pub(crate) welcome_msg: String,
}

// Required for async FromRequest implementations
#[rocket::async_trait]
impl<'r> FromRequest<'r> for IndexConfig {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let agent_header = request.headers().get_one("user-agent");
        if let Some(user_agent) = agent_header {
            let mut variants = default::DefaultVariants::default();
            if user_agent.contains("Edg/") {
                _ = variants.add("browser", "edge");
            }

            let config = variants::de::from_file_with_variants::<IndexConfig, _, _>(
                "./src/config/index.toml",
                &variants,
            );
            Outcome::Success(config.unwrap())
        } else {
            // user-agent header found, forward
            Outcome::Forward(rocket::http::Status { code: 500 })
        }
    }
}
