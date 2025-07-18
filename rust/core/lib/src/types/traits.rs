#![allow(dead_code)]

pub trait Variants: Sized {
    fn matches(&self, variant: &Option<String>) -> bool;
}

impl Variants for std::collections::HashMap<String, String> {
    fn matches(&self, variants: &Option<String>) -> bool {
        match variants {
            None => true,
            Some(str) => {
                let parts = str.split('&');
                for part in parts {
                    if let Some((name, variant)) = part.split_once(':') {
                        if let Some((_, value)) = self.get_key_value(name) {
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
