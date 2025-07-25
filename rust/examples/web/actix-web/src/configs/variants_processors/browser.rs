use actix_web::http::header::HeaderMap;

pub(crate) struct BrowserVaraints {}

impl Default for BrowserVaraints {
    fn default() -> Self {
        Self {}
    }
}

impl variants_actix_web::VariantsProcessor for BrowserVaraints {
    fn process(
        &self,
        request: &actix_web::HttpRequest,
        variants: &mut variants_actix_web::default::DefaultVariants,
    ) {
        let headers: &HeaderMap = request.headers();
        let agent_header = headers.get("User-Agent");
        if let Some(user_agent_value) = agent_header {
            if user_agent_value.to_str().unwrap().contains("Edg/") {
                _ = variants.add("browser", "edge");
            }
        }
    }
}
