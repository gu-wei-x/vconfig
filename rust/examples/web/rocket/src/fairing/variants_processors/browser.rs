use rocket::Request;
pub(crate) struct BrowserVaraints;

impl Default for BrowserVaraints {
    fn default() -> Self {
        Self {}
    }
}

impl variants_rocket::VariantsProcessor for BrowserVaraints {
    fn process<'r>(
        &self,
        request: &'r Request<'_>,
        variants: &mut variants_rocket::default::DefaultVariants,
    ) {
        match request.headers().get_one("sec-ch-ua") {
            Some(sec_ch_ua_value) => {
                let lowwe_cased_value_str = sec_ch_ua_value.to_lowercase();
                if lowwe_cased_value_str.contains("microsoft edge") {
                    _ = variants.add("browser", "edge");
                } else if lowwe_cased_value_str.contains("google chrome") {
                    _ = variants.add("browser", "chrome");
                }
            }
            _ => {
                if let Some(user_agent_value) = request.headers().get_one("user-agent") {
                    let lowwe_cased_value_str = user_agent_value.to_lowercase();
                    if lowwe_cased_value_str.contains("chrome/") {
                        match lowwe_cased_value_str.contains("edg/") {
                            true => {
                                _ = variants.add("browser", "edge");
                            }
                            false => {
                                _ = variants.add("browser", "chrome");
                            }
                        }
                    }
                }
            }
        }
    }
}
