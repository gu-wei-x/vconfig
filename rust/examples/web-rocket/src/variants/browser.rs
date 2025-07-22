#![allow(dead_code, unused_imports)]
extern crate variants as variantslib;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use variantslib::traits::Variants;
use variantslib::traits::VariantsProcessor;

pub(crate) struct BrowserVaraints {}

impl Default for BrowserVaraints {
    fn default() -> Self {
        Self {}
    }
}

impl<'r, V> VariantsProcessor<Request<'r>, V> for BrowserVaraints
where
    V: variantslib::traits::Variants,
{
    fn process(&self, request: &Request<'r>, variants: &mut V) {
        let agent_header = request.headers().get_one("user-agent");
        if let Some(user_agent) = agent_header {
            if user_agent.contains("Edg/") {
                _ = variants.add("browser", "edge");
            }
        }
    }
}
