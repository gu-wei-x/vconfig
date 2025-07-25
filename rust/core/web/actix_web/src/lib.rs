mod builder;
mod context;

pub mod de {
    #[doc(inline)]
    pub use variants::de::from_file_with_variants;

    #[doc(inline)]
    pub use variants_codegen::actix_web_variants_config as variants_config;
}

pub mod default {
    #[doc(inline)]
    pub use variants::default::*;
}

pub mod serde {
    #[doc(inline)]
    pub use variants::serde::*;
}

pub use builder::VariantsProcessor;
pub use context::VaraintsContext;
