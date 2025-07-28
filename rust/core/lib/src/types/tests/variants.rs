#![cfg(test)]
use crate::{default::DefaultVariants, traits::Variants as _};

#[test]
fn test_default_variants() {
    let mut variants = DefaultVariants::default();
    _ = variants.add("variant1", "v1");
    _ = variants.add("VariAnt2", "v2");

    assert!(variants.matches(&Some("variant1:v1".to_string())));
    assert!(variants.matches(&Some("VariAnt1:v1".to_string())));
    assert!(variants.matches(&Some("VariAnt1:v1&variant2:v2".to_string())));
    assert!(variants.matches(&Some("variant1:v1&VariAnt2:v2".to_string())));
}
