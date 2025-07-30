mod builder;
mod context;

pub mod de {
    #[doc(inline)]
    pub use variants_de::de::from_file_with_variants;

    #[doc(inline)]
    pub use variants_codegen::axum_variants_config as variants_config;
}

pub mod default {
    #[doc(inline)]
    pub use variants_de::default::*;
}

pub mod serde {
    #[doc(inline)]
    pub use variants_de::serde::*;
}

pub use builder::VariantsProcessor;
pub use context::VariantsContext;
