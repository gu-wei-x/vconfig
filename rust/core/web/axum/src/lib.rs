mod builder;
mod context;

#[doc(inline)]
pub use builder::VariantsProcessor;

/// `vconfig` context used in axum applications.
#[doc(inline)]
pub use context::VConfigContext;

/// Re-export `vconfig::de::from_file` as `de_from_file`.
#[doc(inline)]
pub use vconfig::de::from_file as de_from_file;

/// Re-export `vconfig_codegen::axum_variant_config` as `vconfig`.
///
/// Example usage:
///
/// ```text
/// #[vconfig("my_config")] // my_config is the name of the configuration file without extension
/// // or #[vconfig(file = "[path]/my_config.toml")]
/// #[derive(Debug)]
/// struct MyConfig {
///     field1: String,
///     field2: i32,
/// }
/// ```
///
/// # `vconfig_codegen::axum_variant_config`
#[doc(inline)]
pub use vconfig_codegen::axum_variant_config as vconfig;

/// Re-export `vconfig::default::DefaultVariants` as `DefaultVariants`.
#[doc(inline)]
pub use vconfig::default::DefaultVariants;

/// Re-export `vconfig::traits::Variants` as `Variants`.
#[doc(inline)]
pub use vconfig::traits::Variants;

/// Re-export `serde`.
pub mod serde {
    #[doc(inline)]
    pub use vconfig::serde::*;
}
