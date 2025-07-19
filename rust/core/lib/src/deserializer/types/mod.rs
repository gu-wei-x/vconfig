use crate::types::traits::Variants;

pub(crate) mod array;
pub(crate) mod array_seqaccess;
pub(crate) mod table;
pub(crate) mod table_mapaccess;
pub(crate) mod value;

pub(crate) struct Deserializer<'a, V, T>
where
    V: Variants,
{
    data: T,
    variants: &'a V,
}

use crate::types::error;
use crate::types::value::Value;

impl error::Error {
    pub(crate) fn from_de(source_type: &str, value: &Value) -> Self {
        error::Error::De(format!("{:?}=>{}", value, source_type))
    }
}
