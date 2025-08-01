use axum::http::request::Parts;
use vconfig_axum::Variants;
use vconfig_axum::VariantsProcessor;

pub(crate) struct BrowserVariants;

impl Default for BrowserVariants {
    fn default() -> Self {
        Self {}
    }
}

impl VariantsProcessor for BrowserVariants {
    fn process(&self, request: &Parts, variants: &mut dyn Variants) {
        match request.headers.get("sec-ch-ua") {
            Some(sec_ch_ua_value) => {
                if let Ok(value_str) = sec_ch_ua_value.to_str() {
                    let lowwe_cased_value_str = value_str.to_lowercase();
                    if lowwe_cased_value_str.contains("microsoft edge") {
                        _ = variants.add("browser", "edge");
                    } else if lowwe_cased_value_str.contains("google chrome") {
                        _ = variants.add("browser", "chrome");
                    }
                }
            }
            _ => {
                if let Some(user_agent_value) = request.headers.get("user-agent") {
                    if let Ok(value_str) = user_agent_value.to_str() {
                        let lowwe_cased_value_str = value_str.to_lowercase();
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
}
