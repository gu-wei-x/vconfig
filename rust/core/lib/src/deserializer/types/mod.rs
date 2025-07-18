use crate::types::traits::Variants;

pub(crate) mod array;
pub(crate) mod table;
pub(crate) mod table_map_access;
pub(crate) mod value;

pub(crate) struct Deserializer<'a, V, T>
where
    V: Variants,
{
    data: T,
    variants: &'a V,
}
