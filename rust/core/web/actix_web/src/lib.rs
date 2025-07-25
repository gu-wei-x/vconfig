mod builder;
mod context;

#[doc(inline)]
pub use variants_codegen::*;

pub mod de {
    #[doc(inline)]
    pub use variants::de::from_file_with_variants;
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
