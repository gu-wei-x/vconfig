mod builder;
mod context;

pub mod de {
    #[doc(inline)]
    pub use vconfig::de::from_file_with_variants;

    #[doc(inline)]
    pub use vconfig_codegen::rocket_variant_config as vconfig;
}

pub mod default {
    #[doc(inline)]
    pub use vconfig::default::*;
}

pub mod serde {
    #[doc(inline)]
    pub use vconfig::serde::*;
}

pub use builder::VariantsProcessor;
pub use context::VariantsContext;
