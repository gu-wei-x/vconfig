extern crate variants as variantslib;
use rocket::Request;
use variantslib::traits::Variants;

pub(crate) struct BrowserVaraints {}

impl Default for BrowserVaraints {
    fn default() -> Self {
        Self {}
    }
}

impl variants_rocket::VariantsProcessor for BrowserVaraints {
    fn process<'r>(
        &self,
        request: &'r Request<'_>,
        variants: &mut variantslib::default::DefaultVariants,
    ) {
        let agent_header = request.headers().get_one("user-agent");
        if let Some(user_agent) = agent_header {
            if user_agent.contains("Edg/") {
                _ = variants.add("browser", "edge");
            }
        }
    }
}
