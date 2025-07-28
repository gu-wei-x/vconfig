pub trait Variants: Sized {
    fn matches(&self, variant: &Option<String>) -> bool;
}

pub struct DefaultVariants {
    variants: std::collections::HashMap<String, String>,
}

use crate::types::error::Error;
use crate::types::result::Result;

impl DefaultVariants {
    pub fn add(&mut self, variant_name: &str, variant_value: &str) -> Result<()> {
        let result = self.variants.insert(
            variant_name.to_lowercase().to_owned(),
            variant_value.to_owned(),
        );
        match result {
            Some(_) => Ok(()),
            None => Error::from_str("varaint exists").into(),
        }
    }
}

impl Default for DefaultVariants {
    fn default() -> Self {
        Self {
            variants: std::collections::HashMap::new(),
        }
    }
}

impl Variants for DefaultVariants {
    fn matches(&self, variants: &Option<String>) -> bool {
        match variants {
            None => true,
            Some(str) => {
                let parts = str.split('&');
                for part in parts {
                    if let Some((name, variant)) = part.split_once(':') {
                        if let Some((_, value)) =
                            self.variants.get_key_value(name.to_lowercase().as_str())
                        {
                            if variant != value {
                                return false;
                            }
                        } else {
                            // no key-value
                            return false;
                        }
                    } else {
                        // invalid input.
                        return false;
                    }
                }
                true
            }
        }
    }
}
