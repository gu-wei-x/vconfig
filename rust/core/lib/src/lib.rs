mod deserializer;
mod parser;
mod types;

use types::result::Result;

use crate::{
    deserializer::Deserializer,
    parser::tokenizer::{self},
};

pub mod traits {
    pub use crate::types::traits::Variants;
}

pub fn from_str_with_variants<'s: 'v, 'v, T, V>(source: &'s str, variants: &'v V) -> Result<T>
where
    T: serde::de::Deserialize<'v>,
    V: types::traits::Variants,
{
    let result = parser::parse_str(source);
    match result {
        Ok(table) => {
            let deserializer = Deserializer::new(table, variants);
            let obj = T::deserialize(deserializer).unwrap();
            Ok(obj)
        }
        Err(err) => Err(err),
    }
}

/*pub fn from_str<'s, T>(source: &'s str) -> Result<T, Token>
where
    T: serde::de::Deserialize<'s>,
{
    let variants: HashMap<String, String> = std::collections::HashMap::new();
    from_str_with_variants(source, variants)
}*/

/*pub fn from_file_with_variants<'s, T>(source: &'s str) -> Result<T, Token>
where
    T: serde::de::Deserialize<'s>,
{
    let variants: HashMap<String, String> = std::collections::HashMap::new();
    from_str_with_variants(source, variants)
}*/


/*pub fn from_file<'s, T>(source: &'s str) -> Result<T, Token>
where
    T: serde::de::Deserialize<'s>,
{
    let variants: HashMap<String, String> = std::collections::HashMap::new();
    from_str_with_variants(source, variants)
}*/
