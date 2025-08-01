mod browser;

pub fn create_variants_context(
    config_dir: &std::path::Path,
) -> Option<vconfig_axum::VariantsContext> {
    if let Some(mut variants_context) = vconfig_axum::VariantsContext::new(config_dir) {
        // add all processor here.
        variants_context.with_processor(browser::BrowserVariants::default());
        Some(variants_context)
    } else {
        None
    }
}
