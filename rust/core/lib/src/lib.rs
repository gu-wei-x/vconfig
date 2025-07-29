mod deserializer;
mod parser;
mod types;

pub mod de {
    use super::types::error;
    use super::types::result::Result;

    pub fn from_str_with_variants<'s, 'v, T, V>(source: &'s str, variants: &'v V) -> Result<T>
    where
        T: serde::de::Deserialize<'v>,
        V: super::types::traits::Variants,
    {
        let parse_result = super::parser::parse_str(source)?;
        let deserializer = super::deserializer::Deserializer::new(parse_result, variants);
        T::deserialize(deserializer)
    }

    pub fn from_file_with_variants<'v, T, P, V>(path: P, variants: &'v V) -> Result<T>
    where
        P: AsRef<std::path::Path>,
        T: serde::de::Deserialize<'v>,
        V: super::types::traits::Variants,
    {
        match &std::fs::read_to_string(path) {
            Ok(source) => from_str_with_variants(source, variants),
            Err(error) => Err(error::Error::from_str(&error.to_string())),
        }
    }
}

pub mod default {
    pub use crate::types::traits::DefaultVariants;
}

pub mod fs;

pub mod serde;

pub mod traits {
    pub use crate::types::traits::Variants;
}
