extern crate variants as variantslib;
use rocket::Request;

pub(crate) struct BrowserVaraints {}

impl Default for BrowserVaraints {
    fn default() -> Self {
        Self {}
    }
}

impl<'r, V> crate::variants::builder::VariantsProcessor<'r, V> for BrowserVaraints
where
    V: variantslib::traits::Variants,
{
    fn process(&self, request: &'r Request<'_>, variants: &mut V) {
        let agent_header = request.headers().get_one("user-agent");
        if let Some(user_agent) = agent_header {
            if user_agent.contains("Edg/") {
                _ = variants.add("browser", "edge");
            }
        }
    }
}
