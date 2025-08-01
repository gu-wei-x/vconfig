mod builder;
mod context;

#[doc(inline)]
pub use builder::VariantsProcessor;

#[doc(inline)]
pub use context::VConfigContext;

#[doc(inline)]
pub use vconfig::de::from_file as de_from_file;

#[doc(inline)]
pub use vconfig_codegen::actix_web_variant_config as vconfig;

#[doc(inline)]
pub use vconfig::default::DefaultVariants;

pub mod serde {
    #[doc(inline)]
    pub use vconfig::serde::*;
}
