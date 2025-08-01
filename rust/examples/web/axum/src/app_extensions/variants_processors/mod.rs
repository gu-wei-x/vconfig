mod browser;

pub fn create_vconfig_context(
    config_dir: &std::path::Path,
) -> Option<vconfig_axum::VConfigContext> {
    if let Some(mut vconfig_context) = vconfig_axum::VConfigContext::new(config_dir) {
        // add all processor here.
        vconfig_context.with_processor(browser::BrowserVariants::default());
        Some(vconfig_context)
    } else {
        None
    }
}
